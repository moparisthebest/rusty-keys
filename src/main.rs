extern crate uinput;
extern crate uinput_sys as ffi;
extern crate libc;
extern crate getopts;

#[macro_use]
extern crate nix;

use uinput::Device;

use ffi::*;
use libc::{c_int, input_event};

use std::thread;
use std::time::Duration;
use std::collections::HashMap;

use std::process::{exit, Command};
use std::fs::File;
use std::io::{Read};
use std::{env, mem};

use std::os::unix::io::AsRawFd;

use getopts::Options;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const EV_KEY_U16 : u16 = EV_KEY as u16;

#[derive(Debug)]
struct Config {
    device_file: String,
    log_file: String
}

impl Config {
    fn new(device_file: String, log_file: String) -> Self {
        Config { device_file: device_file, log_file: log_file }
    }
}

fn main() {

    let key_map = KeyMap::key_map();

    //println!("key_map: {:?}", key_map);

    let device = uinput::default().expect("1")
        .name("test").expect("2")
        .event(key_map.values()).expect("3")
        //.event(uinput::event::Keyboard::All).unwrap()
        .create().expect("4");

    let mut key_map = KeyMap::new();
    key_map.map(KEY_A, KEY_B);

    thread::sleep(Duration::from_secs(1));

    //device.click(EV_KEY, KEY_H).unwrap();
    //device.synchronize().unwrap();

    let config = parse_args();
    println!("Config: {:?}", config);

    let mut input_device = InputDevice::open(&config.device_file);
    input_device.grab();

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
    opts.optopt("d", "device", "specify the device file", "DEVICE");
    opts.optopt("f", "file", "specify the file to log to", "FILE");

    let matches = opts.parse(&args[1..]).unwrap_or_else(|e| panic!("{}", e));
    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        exit(0);
    }

    if matches.opt_present("v") {
        println!("{}", VERSION);
        exit(0);
    }

    let device_file = matches.opt_str("d").unwrap_or_else(|| get_default_device());
    let log_file = matches.opt_str("f").unwrap_or("keys.log".to_owned());

    Config::new(device_file, log_file)
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
const SIZE_OF_INPUT_EVENT : usize = 24;//mem::size_of::<input_event>();

struct InputDevice {
    device_file : File,
    buf : [u8; SIZE_OF_INPUT_EVENT],
}

impl InputDevice {
    pub fn open(device_file : &str) -> Self {
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

// keymapper stuff

trait KeyMapper {
    fn send_event(&self, event: input_event, device: &Device);
}

// 249 is one more than KEY_MICMUTE which is max key in uinput-sys event.rs
const KEY_MAX : usize = 249;

struct KeyMap {
    keymap: [u16; KEY_MAX],
}

impl KeyMap {
    pub fn key_map() -> HashMap<&'static str, *const c_int> {
            [
                ("Reserved", KEY_RESERVED),
                ("Esc", KEY_ESC),
                ("_1", KEY_1),
                ("_2", KEY_2),
                ("_3", KEY_3),
                ("_4", KEY_4),
                ("_5", KEY_5),
                ("_6", KEY_6),
                ("_7", KEY_7),
                ("_8", KEY_8),
                ("_9", KEY_9),
                ("_0", KEY_10),
                ("Minus", KEY_MINUS),
                ("Equal", KEY_EQUAL),
                ("BackSpace", KEY_BACKSPACE),
                ("Tab", KEY_TAB),
                ("Q", KEY_Q),
                ("W", KEY_W),
                ("E", KEY_E),
                ("R", KEY_R),
                ("T", KEY_T),
                ("Y", KEY_Y),
                ("U", KEY_U),
                ("I", KEY_I),
                ("O", KEY_O),
                ("P", KEY_P),
                ("LeftBrace", KEY_LEFTBRACE),
                ("RightBrace", KEY_RIGHTBRACE),
                ("Enter", KEY_ENTER),
                ("LeftControl", KEY_LEFTCTRL),
                ("A", KEY_A),
                ("S", KEY_S),
                ("D", KEY_D),
                ("F", KEY_F),
                ("G", KEY_G),
                ("H", KEY_H),
                ("J", KEY_J),
                ("K", KEY_K),
                ("L", KEY_L),
                ("SemiColon", KEY_SEMICOLON),
                ("Apostrophe", KEY_APOSTROPHE),
                ("Grave", KEY_GRAVE),
                ("LeftShift", KEY_LEFTSHIFT),
                ("BackSlash", KEY_BACKSLASH),
                ("Z", KEY_Z),
                ("X", KEY_X),
                ("C", KEY_C),
                ("V", KEY_V),
                ("B", KEY_B),
                ("N", KEY_N),
                ("M", KEY_M),
                ("Comma", KEY_COMMA),
                ("Dot", KEY_DOT),
                ("Slash", KEY_SLASH),
                ("RightShift", KEY_RIGHTSHIFT),
                ("LeftAlt", KEY_LEFTALT),
                ("Space", KEY_SPACE),
                ("CapsLock", KEY_CAPSLOCK),
                ("F1", KEY_F1),
                ("F2", KEY_F2),
                ("F3", KEY_F3),
                ("F4", KEY_F4),
                ("F5", KEY_F5),
                ("F6", KEY_F6),
                ("F7", KEY_F7),
                ("F8", KEY_F8),
                ("F9", KEY_F9),
                ("F10", KEY_F10),
                ("NumLock", KEY_NUMLOCK),
                ("ScrollLock", KEY_SCROLLLOCK),
                ("F11", KEY_F11),
                ("F12", KEY_F12),
                ("RightControl", KEY_RIGHTCTRL),
                ("SysRq", KEY_SYSRQ),
                ("RightAlt", KEY_RIGHTALT),
                ("LineFeed", KEY_LINEFEED),
                ("Home", KEY_HOME),
                ("Up", KEY_UP),
                ("PageUp", KEY_PAGEUP),
                ("Left", KEY_LEFT),
                ("Right", KEY_RIGHT),
                ("End", KEY_END),
                ("Down", KEY_DOWN),
                ("PageDown", KEY_PAGEDOWN),
                ("Insert", KEY_INSERT),
                ("Delete", KEY_DELETE),
                ("LeftMeta", KEY_LEFTMETA),
                ("RightMeta", KEY_RIGHTMETA),
                ("ScrollUp", KEY_SCROLLUP),
                ("ScrollDown", KEY_SCROLLDOWN),
                ("F13", KEY_F13),
                ("F14", KEY_F14),
                ("F15", KEY_F15),
                ("F16", KEY_F16),
                ("F17", KEY_F17),
                ("F18", KEY_F18),
                ("F19", KEY_F19),
                ("F20", KEY_F20),
                ("F21", KEY_F21),
                ("F22", KEY_F22),
                ("F23", KEY_F23),
                ("F24", KEY_F24),
            ].iter().cloned().map(|(m, v)| (m, v as *const c_int)).collect()
    }

    pub fn new() -> Self {
        let mut keymap = [0u16; KEY_MAX];
        // which is rustier
        /*
        for x  in 0..KEY_MAX {
            keymap[x as usize] = x as u16;
        }
        */
        for (x, v) in keymap.iter_mut().enumerate() {
            *v = x as u16;
        }
        println!("keymap: {:?}", &keymap[..]);
        KeyMap {
            keymap: keymap
        }
    }

    pub fn map(&mut self, from : c_int, to: c_int) {
        self.keymap[from as usize] = to as u16;
    }
}

impl KeyMapper for KeyMap {
    fn send_event(&self, mut event: input_event, device: &Device) {
        event.code = self.keymap[event.code as usize];
        device.write_event(event).expect("could not write event?");
    }
}