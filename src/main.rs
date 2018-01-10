extern crate rusty_keys;
extern crate uinput_sys as ffi;
extern crate libc;
extern crate getopts;
extern crate inotify;

use rusty_keys::{KeyMaps, Device, InputDevice, Result};

use ffi::*;
use libc::input_event;
use std::process::exit;
use std::{env, thread};
use std::sync::mpsc;
use std::sync::mpsc::Sender;

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
    let ret = main_res();
    if let Err(e) = ret {
        println!("fatal error: {}", e);
    }
}

fn main_res() -> Result<()> {
    let config = parse_args();
    //println!("Config: {:?}", config);

    let key_map = KeyMaps::key_map();
    //println!("key_map: {:?}", key_map);

    let device = rusty_keys::open("/dev/uinput")
        .or_else(|_| rusty_keys::open("/dev/input/uinput"))
        .or_else(|_| rusty_keys::default())?
        .name("rusty-keys")?
        .event(key_map.values())?
        .create()?;

    let mut key_map = KeyMaps::from_cfg(&key_map, &config.config_file);
    //println!("keymaps: {:?}", keymaps);

    if config.device_files.len() == 1 {
        // shortcut, don't bother with threads
        let mut input_device = InputDevice::open(&config.device_files[0])?;
        input_device.grab()?;

        loop {
            let event = input_device.read_event()?;
            send_event(&mut key_map, event, &device)?
        }
    } else {
        // start up some intra thread communication
        let (tx, rx) = mpsc::channel();

        if config.device_files.len() > 0 {
            // we only want to operate on device files sent in then quit
            for device_file in config.device_files.iter() {
                let device_file = device_file.clone();
                let tx = tx.clone();
                thread::spawn(move || {
                    let ret = spawn_map_thread(tx, &device_file);
                    if let Err(e) = ret {
                        println!("mapping for {} ended due to error: {}", device_file, e);
                    }
                });
            }
        } else {
            let tx = tx.clone();
            thread::spawn(move || {
                // we want to wait forever starting new threads for any new keyboard devices
                let mut inotify = Inotify::init().expect("Failed to initialize inotify");

                inotify.add_watch("/dev/input/", WatchMask::CREATE).expect("Failed to add inotify watch");

                let device_files = get_keyboard_device_filenames();
                println!("Detected devices: {:?}", device_files);
                for device_file in device_files.iter() {
                    inotify_spawn_thread(&tx, device_file);
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
                                    inotify_spawn_thread(&tx, device_file.clone());
                                }
                            }
                        }
                    }
                }
            });
        }
        drop(tx); // drop our last one, so when the threads finish, everything stops
        // process all events
        for event in rx {
            send_event(&mut key_map, event, &device)?
        }
    }
    Ok(())
}

fn send_event(key_map: &mut KeyMaps, mut event: input_event, device: &Device) -> Result<()> {
    if event.type_ == EV_KEY_U16 {
        key_map.send_event(&mut event, &device)?
    } else {
        device.write_event(&mut event)?
    }
    Ok(())
}

fn inotify_spawn_thread(tx: &Sender<input_event>, device_file: &str) {
    let mut filename = "/dev/input/".to_string();
    filename.push_str(&device_file);
    let tx = tx.clone();
    thread::spawn(move || {
        let ret = spawn_map_thread(tx, &filename);
        if let Err(e) = ret {
            println!("mapping for {} ended due to error: {}", filename, e);
        }
    });
}

fn spawn_map_thread(tx: Sender<input_event>, device_file: &str) -> Result<()> {
    let mut input_device = InputDevice::open(device_file)?;
    input_device.grab()?;

    loop {
        let event = input_device.read_event()?;
        tx.send(event)?
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
    opts.optopt("c", "config", "specify the keymap config file to use (default: /etc/rusty-keys/keymap.toml)", "FILE");

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
// grep -E 'Handlers|EV' /proc/bus/input/devices | grep -B1 120013 | grep -Eo event[0-9]+
fn get_keyboard_device_filenames() -> Vec<String> {
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::fs::File;

    let f = File::open("/proc/bus/input/devices");
    if f.is_err() {
        return Vec::new();
    }
    let f = BufReader::new(f.unwrap());
    let mut filename = None;
    let mut filenames = Vec::new();
    for line in f.lines() {
        if let Ok(line) = line {
            if line.starts_with("H: Handlers=") {
                if let Some(event_index) = line.find("event") {
                    let last_index = line[event_index..line.len()-1].find(" ").and_then(|i| Some(i + event_index)).unwrap_or(line.len() - 1);
                    filename = Some(line[event_index..last_index].to_owned());
                }
            } else if line.starts_with("B: EV=") && line.contains("120013") {
                if let Some(ref filename) = filename {
                    filenames.push(filename.clone());
                }
            }
        }
    }
    filenames
}