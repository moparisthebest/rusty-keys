extern crate rusty_keys;
extern crate uinput_sys as ffi;
extern crate libc;
extern crate getopts;
extern crate inotify;

#[macro_use]
extern crate nix;

use rusty_keys::KeyMaps;

use ffi::*;
use libc::{c_int, input_event};

//use std::thread;
//use std::time::Duration;

use std::process::{exit, Command};
use std::fs::File;
use std::io::Read;
use std::{env, mem};
use std::thread;
use std::sync::{Arc, mpsc};
use std::collections::HashMap;

use std::os::unix::io::AsRawFd;

use getopts::Options;

use inotify::{
    EventMask,
    Inotify,
    WatchMask,
};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const EV_KEY_U16: u16 = EV_KEY as u16;

#[derive(Debug)]
struct Config {
    device_files: Vec<String>,
    config_file: String
}

impl Config {
    fn new(device_files: Vec<String>, config_file: String) -> Self {
        Config { device_files: device_files, config_file: config_file }
    }
}

fn main() {
    let config = parse_args();
    //println!("Config: {:?}", config);

    let key_map = Arc::new(KeyMaps::key_map());
    //println!("key_map: {:?}", key_map);

    if config.device_files.len() > 0 {
        // we only want to operate on device files sent in then quit
        let (tx, rx) = mpsc::channel();

        for device_file in config.device_files.iter() {
            let device_file = device_file.clone();
            let config_file = config.config_file.clone();
            let tx = tx.clone();
            let key_map = Arc::clone(&key_map);
            thread::spawn(move || {
                let ret = spawn_map_thread(key_map, &device_file, &config_file);
                if let Err(e) = ret {
                    println!("mapping for {} ended due to error: {}", device_file, e);
                }
                tx.send(1).ok();
            });
        }
        // wait for all threads to finish
        let mut num_threads = config.device_files.len();
        for received in rx {
            num_threads -= received;
            if num_threads == 0 {
                break;
            }
        }
    } else {
        // we want to wait forever starting new threads for any new keyboard devices
        let mut inotify = Inotify::init().expect("Failed to initialize inotify");

        inotify.add_watch("/dev/input/", WatchMask::CREATE).expect("Failed to add inotify watch");

        let device_files = get_keyboard_device_filenames();
        println!("Detected devices: {:?}", device_files);
        for device_file in device_files.iter() {
            inotify_spawn_thread(&key_map, device_file, config.config_file.clone());
        }

        let mut buffer = [0u8; 4096];
        loop {
            let events = inotify.read_events_blocking(&mut buffer);

            if let Ok(events) = events {
                for event in events {
                    if !event.mask.contains(EventMask::ISDIR) {
                        if let Some(device_file) = event.name.and_then(|name|name.to_str()) {
                            // check if this is an eligible keyboard device
                            let device_files = get_keyboard_device_filenames();
                            if !device_files.contains(&device_file.to_string()) {
                                continue;
                            }
                            println!("starting mapping thread for: {}", device_file);
                            inotify_spawn_thread(&key_map, device_file.clone(), config.config_file.clone());
                        }
                    }
                }
            }
        }
    }
}

fn inotify_spawn_thread(key_map: &Arc<HashMap<&'static str, c_int>>, device_file: &str, config_file: String) {
    let mut filename = "/dev/input/".to_string();
    filename.push_str(&device_file);
    let key_map = Arc::clone(&key_map);
    thread::spawn(move || {
        let ret = spawn_map_thread(key_map, &filename, &config_file);
        if let Err(e) = ret {
            println!("mapping for {} ended due to error: {}", filename, e);
        }
    });
}

fn spawn_map_thread(key_map: Arc<HashMap<&'static str, c_int>>, device_file: &str, config_file: &str) -> Result<()> {
    let mut input_device = InputDevice::open(device_file)?;
    input_device.grab()?;

    let device = rusty_keys::open("/dev/uinput")
        .or_else(|_| rusty_keys::open("/dev/input/uinput"))
        .or_else(|_| rusty_keys::default())?
        .name("rusty-keys")?
        .event(key_map.values())?
        .create()?;

    let mut key_map = KeyMaps::from_cfg(&key_map, config_file);
    //println!("keymaps: {:?}", keymaps);

    loop {
        let mut event = input_device.read_event()?;
        if event.type_ == EV_KEY_U16 {
            key_map.send_event(&mut event, &device)?
        } else {
            device.write_event(&mut event)?
        }
    }
}

fn parse_args() -> Config {
    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} [options] [device_files...]", program);
        println!("{}", opts.usage(&brief));
    }

    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "prints this help message");
    opts.optflag("v", "version", "prints the version");
    opts.optopt("c", "config", "specify the keymap config file to use", "FILE");

    let matches = opts.parse(&args[1..]);
    if matches.is_err() {
        print_usage(&args[0], opts);
        exit(0);
    }
    let matches = matches.unwrap();
    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        exit(0);
    }

    if matches.opt_present("v") {
        println!("rusty-keys {}", VERSION);
        exit(0);
    }

    let config_file = matches.opt_str("c").unwrap_or("/etc/rusty-keys/keymap.toml".to_owned());

    Config::new(matches.free, config_file)
}

// Detects and returns the name of the keyboard device file. This function uses
// the fact that all device information is shown in /proc/bus/input/devices and
// the keyboard device file should always have an EV of 120013
fn get_keyboard_device_filenames() -> Vec<String> {
    let command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices | grep -B1 120013 | grep -Eo event[0-9]+".to_string();
    let res = Command::new("sh").arg("-c").arg(command_str).output();
    if res.is_err() {
        return Vec::new();
    }
    let res = res.unwrap();
    let res_str = std::str::from_utf8(&res.stdout).unwrap_or("");

    let mut filenames = Vec::new();
    for file in res_str.trim().split('\n') {
        filenames.push(file.to_string());
    }
    filenames
}

// inputdevice stuff

ioctl!(write eviocgrab   with b'E', 0x90; c_int);

// TODO: use size_of_input_event instead of hard-coding 24.
const SIZE_OF_INPUT_EVENT: usize = 24;//mem::size_of::<input_event>();

struct InputDevice {
    device_file: File,
    buf: [u8; SIZE_OF_INPUT_EVENT],
}

use rusty_keys::{Error,Result};

impl InputDevice {
    pub fn open(device_file: &str) -> Result<Self> {
        let device_file = File::open(device_file)?;
        Ok(InputDevice {
            device_file: device_file,
            buf: [0u8; SIZE_OF_INPUT_EVENT],
        })
    }

    pub fn read_event(&mut self) -> Result<input_event> {
        let num_bytes = self.device_file.read(&mut self.buf)?;
        if num_bytes != SIZE_OF_INPUT_EVENT {
            return Err(Error::ShortRead);
        }
        let event: input_event = unsafe { mem::transmute(self.buf) };
        Ok(event)
    }

    pub fn grab(&mut self) -> Result<()> {
        unsafe {
            eviocgrab(self.device_file.as_raw_fd(), 1 as *const c_int)?;
        }
        Ok(())
    }

    pub fn release(&mut self) -> Result<()> {
        unsafe {
            eviocgrab(self.device_file.as_raw_fd(), 0 as *const c_int)?;
        }
        Ok(())
    }
}

impl Drop for InputDevice {
    fn drop(&mut self) {
        self.release().ok();
    }
}


