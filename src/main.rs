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

use std::os::unix::io::AsRawFd;

use getopts::Options;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const EV_KEY_U16: u16 = EV_KEY as u16;

#[derive(Debug)]
struct Config {
    device_file: String,
    config_file: String
}

impl Config {
    fn new(device_file: String, config_file: String) -> Self {
        Config { device_file: device_file, config_file: config_file }
    }
}

fn main() {
    let key_map = KeyMaps::key_map();
    //println!("key_map: {:?}", key_map);

    let device = rusty_keys::default().expect("1")
        .name("test").expect("2")
        .event(key_map.values()).expect("3")
        //.event(uinput::event::Keyboard::All).unwrap()
        .create().expect("4");

    //thread::sleep(Duration::from_secs(1));

    let config = parse_args();
    //println!("Config: {:?}", config);

    let mut input_device = InputDevice::open(&config.device_file);
    input_device.grab();

    let mut key_map = KeyMaps::from_cfg(&key_map, config.config_file);
    //println!("keymaps: {:?}", keymaps);

    loop {
        let event = input_device.read_event();
        if event.type_ == EV_KEY_U16 {
            key_map.send_event(event, &device);
            /*
                println!("type: {} code: {}", event.type_, event.code);
                if event.code == KEY_A as u16 {
                    event.code = KEY_B as u16;
                }
                */
        } else {
            device.write_event(event).expect("could not write event?");
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
    opts.optopt("d", "device", "specify the keyboard input device file", "DEVICE");
    opts.optopt("c", "config", "specify the keymap config file to use", "FILE");

    let matches = opts.parse(&args[1..]).unwrap_or_else(|e| panic!("{}", e));
    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        exit(0);
    }

    if matches.opt_present("v") {
        println!("rusty-keys {}", VERSION);
        exit(0);
    }

    let device_file = matches.opt_str("d").unwrap_or_else(|| get_default_device());
    let config_file = matches.opt_str("c").unwrap_or("keymap.toml".to_owned());

    Config::new(device_file, config_file)
}

fn get_default_device() -> String {
    let mut filenames = get_keyboard_device_filenames();
    println!("Detected devices: {:?}", filenames);

    if filenames.len() == 1 {
        filenames.swap_remove(0)
    } else {
        panic!("The following keyboard devices were detected: {:?}. Please select one using \
                the `-d` flag", filenames);
    }
}

// Detects and returns the name of the keyboard device file. This function uses
// the fact that all device information is shown in /proc/bus/input/devices and
// the keyboard device file should always have an EV of 120013
fn get_keyboard_device_filenames() -> Vec<String> {
    let mut command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices".to_string();
    command_str.push_str("| grep -B1 120013");
    command_str.push_str("| grep -Eo event[0-9]+");

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

impl InputDevice {
    pub fn open(device_file: &str) -> Self {
        let device_file = File::open(device_file).unwrap_or_else(|e| panic!("{}", e));
        InputDevice {
            device_file: device_file,
            buf: [0u8; SIZE_OF_INPUT_EVENT],
        }
    }

    pub fn read_event(&mut self) -> input_event {
        let num_bytes = self.device_file.read(&mut self.buf).unwrap_or_else(|e| panic!("{}", e));
        if num_bytes != SIZE_OF_INPUT_EVENT {
            panic!("Error while reading from device file");
        }
        let event: input_event = unsafe { mem::transmute(self.buf) };
        event
    }

    pub fn grab(&mut self) {
        unsafe {
            eviocgrab(self.device_file.as_raw_fd(), 1 as *const c_int).expect("no grab?");
        }
    }

    pub fn release(&mut self) {
        unsafe {
            eviocgrab(self.device_file.as_raw_fd(), 0 as *const c_int).expect("no release?");
        }
    }
}

impl Drop for InputDevice {
    fn drop(&mut self) {
        self.release();
    }
}


