extern crate rusty_keys;
extern crate uinput_sys as ffi;
extern crate libc;
extern crate getopts;

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
use std::sync::mpsc;

use std::os::unix::io::AsRawFd;

use getopts::Options;

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
    let (tx, rx) = mpsc::channel();

    for device_file in config.device_files.iter() {
        let device_file = device_file.clone();
        let config_file = config.config_file.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            let key_map = KeyMaps::key_map();
            //println!("key_map: {:?}", key_map);

            let device = rusty_keys::open("/dev/uinput")
                .or_else(|_| rusty_keys::open("/dev/input/uinput"))
                .or_else(|_| rusty_keys::default())
                .expect("cannot open uinput device")
                .name("test").expect("cannot name uinput device")
                .event(key_map.values()).expect("cannot register events on uinput device")
                .create().expect("cannot create uinput device");

            let mut input_device = InputDevice::open(&device_file);
            input_device.grab();

            let mut key_map = KeyMaps::from_cfg(&key_map, config_file);
            //println!("keymaps: {:?}", keymaps);

            loop {
                let mut event = if let Ok(e) = input_device.read_event() { e } else { break };
                if event.type_ == EV_KEY_U16 {
                    key_map.send_event(&mut event, &device);
                } else {
                    if device.write_event(&mut event).is_err() {
                        break;
                    }
                }
            }
            tx.send(1).unwrap();
        });
    }
    // wait for all threads to finish
    let mut num_threads = config.device_files.len();
    for received in rx {
        println!("Got: {}", received);
        num_threads = num_threads - received;
        if num_threads == 0 {
            break;
        }
    }
}

fn parse_args() -> Config {
    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} [options]", program);
        println!("{}", opts.usage(&brief));
    }

    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "prints this help message");
    opts.optflag("v", "version", "prints the version");
    opts.optopt("c", "config", "specify the keymap config file to use", "FILE");
    opts.optmulti("d", "device", "specify the keyboard input device file", "DEVICE");

    let matches = opts.parse(&args[1..]).unwrap_or_else(|e| panic!("{}", e));
    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        exit(0);
    }

    if matches.opt_present("v") {
        println!("rusty-keys {}", VERSION);
        exit(0);
    }

    let mut device_files = matches.opt_strs("d");
    if device_files.len() == 0 {
        device_files = get_keyboard_device_filenames();
    }
    println!("Detected devices: {:?}", device_files);
    let config_file = matches.opt_str("c").unwrap_or("keymap.toml".to_owned());

    Config::new(device_files, config_file)
}

// Detects and returns the name of the keyboard device file. This function uses
// the fact that all device information is shown in /proc/bus/input/devices and
// the keyboard device file should always have an EV of 120013
fn get_keyboard_device_filenames() -> Vec<String> {
    let command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices | grep -B1 120013 | grep -Eo event[0-9]+".to_string();
    let res = Command::new("sh").arg("-c").arg(command_str).output().unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let res_str = std::str::from_utf8(&res.stdout).unwrap();

    let mut filenames = Vec::new();
    for file in res_str.trim().split('\n') {
        let mut filename = "/dev/input/".to_string();
        filename.push_str(file);
        filenames.push(filename);
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

#[cfg(feature = "udev")]
extern crate libudev as udev;
mod error;
pub use error::Error;
type Result<T> = ::std::result::Result<T, Error>;

impl InputDevice {
    pub fn open(device_file: &str) -> Self {
        let device_file = File::open(device_file).unwrap_or_else(|e| panic!("{}", e));
        InputDevice {
            device_file: device_file,
            buf: [0u8; SIZE_OF_INPUT_EVENT],
        }
    }

    pub fn read_event(&mut self) -> Result<input_event> {
        let num_bytes = self.device_file.read(&mut self.buf)?;
        if num_bytes != SIZE_OF_INPUT_EVENT {
            return Err(Error::ShortRead);
        }
        let event: input_event = unsafe { mem::transmute(self.buf) };
        Ok(event)
    }

    pub fn grab(&mut self) {
        unsafe {
            eviocgrab(self.device_file.as_raw_fd(), 1 as *const c_int).expect("no grab?");
        }
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


