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

    let key_map_config = parse_cfg("keymap.toml").expect("provided config cannot be found/parsed");

    //println!("key_map_config: {:?}", key_map_config);

    let key_map = KeyMap::key_map();

    //println!("key_map: {:?}", key_map);

    let device = uinput::default().expect("1")
        .name("test").expect("2")
        .event(key_map.values()).expect("3")
        //.event(uinput::event::Keyboard::All).unwrap()
        .create().expect("4");

    let mut key_map = KeyMaps::new(&key_map, key_map_config);
    //println!("keymaps: {:?}", keymaps);

    //let mut key_map = KeyMap::new();
    //key_map.map(KEY_A, KEY_B);

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
// 1 is down, 0 is up
const DOWN : i32 = 1;
const UP : i32 = 0;
const INVERT_KEY_FLAG : char = '^';
const CAPS_MODIFY_KEY_FLAG : char = '*';
const HALF_KEY_SEPARATOR: char = ':';

const LEFTSHIFT_INDEX : usize = KEY_LEFTSHIFT as usize;
const RIGHTSHIFT_INDEX : usize = KEY_RIGHTSHIFT as usize;
const CAPSLOCK_INDEX : usize = KEY_CAPSLOCK as usize;

const KEY_LEFTSHIFT_U16 : u16 = KEY_LEFTSHIFT as u16;
const KEY_RIGHTSHIFT_U16 : u16 = KEY_RIGHTSHIFT as u16;
const KEY_CAPSLOCK_U16 : u16 = KEY_CAPSLOCK as u16;

trait KeyMapper {
    fn send_event(&mut self, key_state: &mut [bool], event: input_event, device: &Device);
}

struct KeyMaps {
    keymaps:  Vec<Box<KeyMapper>>,
    keymap_index_keys: HashMap<u16, usize>,
    switch_layout_keys: Vec<usize>,
    key_state: [bool; KEY_MAX],
    revert_default_key: u16,
    revert_keymap_index: usize,
    // above do not change, below does
    chosen_keymap_index: usize,
    current_keymap_index: usize,
}

fn parse_key(key_map: &HashMap<&'static str, *const c_int>, key: &str) -> u16 {
    match key_map.get(key.trim_matches(|c : char| c.is_whitespace() || c == INVERT_KEY_FLAG || c == CAPS_MODIFY_KEY_FLAG)) {
        Some(key_code) => *key_code as u16,
        None => panic!("unknown key: {}", key.trim())
    }
}

fn parse_keymap_numeric(key_map: &HashMap<&'static str, *const c_int>, keymap: &str) -> Vec<u16> {
    keymap.split(",").map(|k|parse_key(key_map, k)).collect()
}

fn parse_key_half_inverted(key_map: &HashMap<&'static str, *const c_int>, key: &str) -> HalfInvertedKey {
    HalfInvertedKey {
        code: parse_key(key_map, key),
        invert_shift: key.contains(INVERT_KEY_FLAG),
        capslock_nomodify: key.contains(CAPS_MODIFY_KEY_FLAG),
    }
}

fn parse_keymap(key_map: &HashMap<&'static str, *const c_int>, keymap: &str) -> Vec<Box<KeyMapper + 'static>> {
    keymap.split(",").map(|k|Box::new(parse_key(key_map, k)) as Box<KeyMapper>).collect()
}

impl KeyMaps {
    pub fn new(key_map: &HashMap<&'static str, *const c_int>, config: KeymapConfig) -> KeyMaps {
        if config.keymaps.len() < 2 {
            panic!("must have at least 2 keymaps (original and mapped) but only have {},", config.keymaps.len());
        }
        if config.default_keymap_index >= config.keymaps.len() || config.revert_keymap_index >= config.keymaps.len() {
            panic!("default_keymap_index ({}) and revert_keymap_index ({}) must be less than keymaps length ({}),", config.default_keymap_index, config.revert_keymap_index, config.keymaps.len());
        }
        let base_keymap = parse_keymap_numeric(key_map, &config.keymaps[0]);
        println!("base_keymap      : {:?}", base_keymap);
        let mut keymaps : Vec<Box<KeyMapper>>= vec!(Box::new(NOOP)); // todo: can we share the box?
        let mut keymap_index_keys: HashMap<u16, usize> = HashMap::new();
        for (x, v) in config.keymaps.iter().enumerate() {
            keymap_index_keys.insert(*key_map.get(&*x.to_string()).unwrap() as u16, x);
            if x == 0 {
                continue;
            }
                let v = v.split(",").map(|k| {

                    let ret : Box<KeyMapper> = if k.contains(HALF_KEY_SEPARATOR) {
                        let keys : Vec<&str> = k.split(HALF_KEY_SEPARATOR).collect();
                        if keys.len() != 2 {
                            panic!("split key can only have 2 keys, 1 :, has {} keys", keys.len());
                        }
                        let mut shift_half = parse_key_half_inverted(key_map, keys[1]);
                        shift_half.invert_shift = !shift_half.invert_shift;
                        Box::new(ShiftInvertedKey {
                            noshift_half: parse_key_half_inverted(key_map, keys[0]),
                            shift_half: shift_half,
                        })
                    } else if k.contains(INVERT_KEY_FLAG) || k.contains(CAPS_MODIFY_KEY_FLAG) {
                        Box::new(parse_key_half_inverted(key_map, k))
                    } else {
                        Box::new(parse_key(key_map, k))
                    };
                    ret
                });//parse_keymap(key_map, v);
                //println!("config.keymaps[{}]: {:?}", x, v);
            /*
                if v.len() != base_keymap.len() {
                    panic!("all keymaps must be the same length, keymap index 0 length: {}, index {} length: {},", base_keymap.len(), x, v.len());
                }
                */
                let mut keymap = KeyMap::new();
            /*
                for(i, key_code) in v.iter().enumerate() {
                    let ptr = Box::into_raw(*key_code);
                    //keymap.map(base_keymap[i], &key_code);
                }

            for(i, key_code) in base_keymap.iter().enumerate() {
                //let ptr = Box::into_raw(*key_code);
                keymap.map(*key_code, v[i]);
            }
            */
            let mut i : usize = 0;
            for key_code in v {
                keymap.map(base_keymap[i], key_code);
                i = i + 1;
                if i > base_keymap.len() {
                    panic!("all keymaps must be the same length, keymap index 0 length: {}, index {} length: {},", base_keymap.len(), x, i);
                }
            }

                //println!("keymap[{}]: {:?}", x, &keymap.keymap[..]);
                keymaps.push(Box::new(keymap));
        }
        //println!("keymaps: {:?}", keymaps);
        //println!("keymap_index_keys: {:?}", keymap_index_keys);

        KeyMaps {
            keymaps: keymaps,
            keymap_index_keys: keymap_index_keys,
            switch_layout_keys: config.switch_layout_keys.iter().map(|k|parse_key(key_map, k) as usize).collect(),
            key_state: [false; KEY_MAX], // todo: detect key state? at least CAPSLOCK...
            revert_default_key: parse_key(key_map, &config.revert_default_key),
            revert_keymap_index: config.revert_keymap_index,
            chosen_keymap_index: config.default_keymap_index,
            current_keymap_index: config.default_keymap_index,
        }
    }
}

//impl KeyMapper for KeyMaps {
impl KeyMaps {
    fn send_event(&mut self, event: input_event, device: &Device) {
        //println!("type: {} code: {} value: {}", event.type_, event.code, event.value);
        if event.value != 2 {
            // todo: index check here...
            if event.code == KEY_CAPSLOCK_U16 {
                if event.value == DOWN {
                    self.key_state[CAPSLOCK_INDEX] = !self.key_state[CAPSLOCK_INDEX];
                }
            } else {
                self.key_state[event.code as usize] = event.value == DOWN;
            }
            let mut switch_layout_keys_pressed = true;
            for mut layout_switch_key in self.switch_layout_keys.iter_mut() {
                if !self.key_state[*layout_switch_key] {
                    switch_layout_keys_pressed = false;
                    break;
                }
            }
            //println!("switch_layout_keys_pressed: {}", self.switch_layout_keys_pressed);
            if switch_layout_keys_pressed {
                let new_index = self.keymap_index_keys.get(&event.code);
                if new_index.is_some() {
                    self.chosen_keymap_index = *new_index.unwrap();
                    self.current_keymap_index = self.chosen_keymap_index; // todo: what if revert_default_key is held? for now ignore
                    return; // we don't want to also send this keypress, so bail
                }
            }
            if event.code == self.revert_default_key {
                match event.value {
                    // todo: ctrl+c will get c stuck because code c value 1 will be sent, but then we'll let go of ctrl, and code j value 0 is sent, so c is never released... fix that...
                    DOWN => self.current_keymap_index = self.revert_keymap_index,
                    UP => self.current_keymap_index = self.chosen_keymap_index,
                    _ => () // do nothing for 2
                }
            }
        }
        self.keymaps[self.current_keymap_index].send_event(&mut self.key_state, event, device);
    }
}

// 249 is one more than KEY_MICMUTE which is max key in uinput-sys event.rs
const KEY_MAX : usize = 249;

struct KeyMap {
    keymap: Vec<Box<KeyMapper>>,//[Box<KeyMapper>; KEY_MAX],
}

impl KeyMap {
    pub fn key_map() -> HashMap<&'static str, *const c_int> {
            [
                // generated like:
                // grep -o 'KEY_[^ :;]*' ~/.cargo/registry/src/github.com-1ecc6299db9ec823/uinput-sys-0.1.3/src/events.rs | sed 's/^KEY_//' | awk '{print "(\""$1"\", KEY_"$1"),"}'
                ("RESERVED", KEY_RESERVED),
                ("ESC", KEY_ESC),
                ("1", KEY_1),
                ("2", KEY_2),
                ("3", KEY_3),
                ("4", KEY_4),
                ("5", KEY_5),
                ("6", KEY_6),
                ("7", KEY_7),
                ("8", KEY_8),
                ("9", KEY_9),
                ("10", KEY_10),
                ("MINUS", KEY_MINUS),
                ("EQUAL", KEY_EQUAL),
                ("BACKSPACE", KEY_BACKSPACE),
                ("TAB", KEY_TAB),
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
                ("LEFTBRACE", KEY_LEFTBRACE),
                ("RIGHTBRACE", KEY_RIGHTBRACE),
                ("ENTER", KEY_ENTER),
                ("LEFTCTRL", KEY_LEFTCTRL),
                ("A", KEY_A),
                ("S", KEY_S),
                ("D", KEY_D),
                ("F", KEY_F),
                ("G", KEY_G),
                ("H", KEY_H),
                ("J", KEY_J),
                ("K", KEY_K),
                ("L", KEY_L),
                ("SEMICOLON", KEY_SEMICOLON),
                ("APOSTROPHE", KEY_APOSTROPHE),
                ("GRAVE", KEY_GRAVE),
                ("LEFTSHIFT", KEY_LEFTSHIFT),
                ("BACKSLASH", KEY_BACKSLASH),
                ("Z", KEY_Z),
                ("X", KEY_X),
                ("C", KEY_C),
                ("V", KEY_V),
                ("B", KEY_B),
                ("N", KEY_N),
                ("M", KEY_M),
                ("COMMA", KEY_COMMA),
                ("DOT", KEY_DOT),
                ("SLASH", KEY_SLASH),
                ("RIGHTSHIFT", KEY_RIGHTSHIFT),
                ("KPASTERISK", KEY_KPASTERISK),
                ("LEFTALT", KEY_LEFTALT),
                ("SPACE", KEY_SPACE),
                ("CAPSLOCK", KEY_CAPSLOCK),
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
                ("NUMLOCK", KEY_NUMLOCK),
                ("SCROLLLOCK", KEY_SCROLLLOCK),
                ("KP7", KEY_KP7),
                ("KP8", KEY_KP8),
                ("KP9", KEY_KP9),
                ("KPMINUS", KEY_KPMINUS),
                ("KP4", KEY_KP4),
                ("KP5", KEY_KP5),
                ("KP6", KEY_KP6),
                ("KPPLUS", KEY_KPPLUS),
                ("KP1", KEY_KP1),
                ("KP2", KEY_KP2),
                ("KP3", KEY_KP3),
                ("KP0", KEY_KP0),
                ("KPDOT", KEY_KPDOT),
                ("ZENKAKUHANKAKU", KEY_ZENKAKUHANKAKU),
                ("102ND", KEY_102ND),
                ("F11", KEY_F11),
                ("F12", KEY_F12),
                ("RO", KEY_RO),
                ("KATAKANA", KEY_KATAKANA),
                ("HIRAGANA", KEY_HIRAGANA),
                ("HENKAN", KEY_HENKAN),
                ("KATAKANAHIRAGANA", KEY_KATAKANAHIRAGANA),
                ("MUHENKAN", KEY_MUHENKAN),
                ("KPJPCOMMA", KEY_KPJPCOMMA),
                ("KPENTER", KEY_KPENTER),
                ("RIGHTCTRL", KEY_RIGHTCTRL),
                ("KPSLASH", KEY_KPSLASH),
                ("SYSRQ", KEY_SYSRQ),
                ("RIGHTALT", KEY_RIGHTALT),
                ("LINEFEED", KEY_LINEFEED),
                ("HOME", KEY_HOME),
                ("UP", KEY_UP),
                ("PAGEUP", KEY_PAGEUP),
                ("LEFT", KEY_LEFT),
                ("RIGHT", KEY_RIGHT),
                ("END", KEY_END),
                ("DOWN", KEY_DOWN),
                ("PAGEDOWN", KEY_PAGEDOWN),
                ("INSERT", KEY_INSERT),
                ("DELETE", KEY_DELETE),
                ("MACRO", KEY_MACRO),
                ("MUTE", KEY_MUTE),
                ("VOLUMEDOWN", KEY_VOLUMEDOWN),
                ("VOLUMEUP", KEY_VOLUMEUP),
                ("POWER", KEY_POWER),
                ("KPEQUAL", KEY_KPEQUAL),
                ("KPPLUSMINUS", KEY_KPPLUSMINUS),
                ("PAUSE", KEY_PAUSE),
                ("SCALE", KEY_SCALE),
                ("KPCOMMA", KEY_KPCOMMA),
                ("HANGEUL", KEY_HANGEUL),
                ("HANGUEL", KEY_HANGUEL),
                ("HANGEUL", KEY_HANGEUL),
                ("HANJA", KEY_HANJA),
                ("YEN", KEY_YEN),
                ("LEFTMETA", KEY_LEFTMETA),
                ("RIGHTMETA", KEY_RIGHTMETA),
                ("COMPOSE", KEY_COMPOSE),
                ("STOP", KEY_STOP),
                ("AGAIN", KEY_AGAIN),
                ("PROPS", KEY_PROPS),
                ("UNDO", KEY_UNDO),
                ("FRONT", KEY_FRONT),
                ("COPY", KEY_COPY),
                ("OPEN", KEY_OPEN),
                ("PASTE", KEY_PASTE),
                ("FIND", KEY_FIND),
                ("CUT", KEY_CUT),
                ("HELP", KEY_HELP),
                ("MENU", KEY_MENU),
                ("CALC", KEY_CALC),
                ("SETUP", KEY_SETUP),
                ("SLEEP", KEY_SLEEP),
                ("WAKEUP", KEY_WAKEUP),
                ("FILE", KEY_FILE),
                ("SENDFILE", KEY_SENDFILE),
                ("DELETEFILE", KEY_DELETEFILE),
                ("XFER", KEY_XFER),
                ("PROG1", KEY_PROG1),
                ("PROG2", KEY_PROG2),
                ("WWW", KEY_WWW),
                ("MSDOS", KEY_MSDOS),
                ("COFFEE", KEY_COFFEE),
                ("SCREENLOCK", KEY_SCREENLOCK),
                ("COFFEE", KEY_COFFEE),
                ("ROTATE_DISPLAY", KEY_ROTATE_DISPLAY),
                ("DIRECTION", KEY_DIRECTION),
                ("ROTATE_DISPLAY", KEY_ROTATE_DISPLAY),
                ("CYCLEWINDOWS", KEY_CYCLEWINDOWS),
                ("MAIL", KEY_MAIL),
                ("BOOKMARKS", KEY_BOOKMARKS),
                ("COMPUTER", KEY_COMPUTER),
                ("BACK", KEY_BACK),
                ("FORWARD", KEY_FORWARD),
                ("CLOSECD", KEY_CLOSECD),
                ("EJECTCD", KEY_EJECTCD),
                ("EJECTCLOSECD", KEY_EJECTCLOSECD),
                ("NEXTSONG", KEY_NEXTSONG),
                ("PLAYPAUSE", KEY_PLAYPAUSE),
                ("PREVIOUSSONG", KEY_PREVIOUSSONG),
                ("STOPCD", KEY_STOPCD),
                ("RECORD", KEY_RECORD),
                ("REWIND", KEY_REWIND),
                ("PHONE", KEY_PHONE),
                ("ISO", KEY_ISO),
                ("CONFIG", KEY_CONFIG),
                ("HOMEPAGE", KEY_HOMEPAGE),
                ("REFRESH", KEY_REFRESH),
                ("EXIT", KEY_EXIT),
                ("MOVE", KEY_MOVE),
                ("EDIT", KEY_EDIT),
                ("SCROLLUP", KEY_SCROLLUP),
                ("SCROLLDOWN", KEY_SCROLLDOWN),
                ("KPLEFTPAREN", KEY_KPLEFTPAREN),
                ("KPRIGHTPAREN", KEY_KPRIGHTPAREN),
                ("NEW", KEY_NEW),
                ("REDO", KEY_REDO),
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
                ("PLAYCD", KEY_PLAYCD),
                ("PAUSECD", KEY_PAUSECD),
                ("PROG3", KEY_PROG3),
                ("PROG4", KEY_PROG4),
                ("DASHBOARD", KEY_DASHBOARD),
                ("SUSPEND", KEY_SUSPEND),
                ("CLOSE", KEY_CLOSE),
                ("PLAY", KEY_PLAY),
                ("FASTFORWARD", KEY_FASTFORWARD),
                ("BASSBOOST", KEY_BASSBOOST),
                ("PRINT", KEY_PRINT),
                ("HP", KEY_HP),
                ("CAMERA", KEY_CAMERA),
                ("SOUND", KEY_SOUND),
                ("QUESTION", KEY_QUESTION),
                ("EMAIL", KEY_EMAIL),
                ("CHAT", KEY_CHAT),
                ("SEARCH", KEY_SEARCH),
                ("CONNECT", KEY_CONNECT),
                ("FINANCE", KEY_FINANCE),
                ("SPORT", KEY_SPORT),
                ("SHOP", KEY_SHOP),
                ("ALTERASE", KEY_ALTERASE),
                ("CANCEL", KEY_CANCEL),
                ("BRIGHTNESSDOWN", KEY_BRIGHTNESSDOWN),
                ("BRIGHTNESSUP", KEY_BRIGHTNESSUP),
                ("MEDIA", KEY_MEDIA),
                ("SWITCHVIDEOMODE", KEY_SWITCHVIDEOMODE),
                ("KBDILLUMTOGGLE", KEY_KBDILLUMTOGGLE),
                ("KBDILLUMDOWN", KEY_KBDILLUMDOWN),
                ("KBDILLUMUP", KEY_KBDILLUMUP),
                ("SEND", KEY_SEND),
                ("REPLY", KEY_REPLY),
                ("FORWARDMAIL", KEY_FORWARDMAIL),
                ("SAVE", KEY_SAVE),
                ("DOCUMENTS", KEY_DOCUMENTS),
                ("BATTERY", KEY_BATTERY),
                ("BLUETOOTH", KEY_BLUETOOTH),
                ("WLAN", KEY_WLAN),
                ("UWB", KEY_UWB),
                ("UNKNOWN", KEY_UNKNOWN),
                ("VIDEO_NEXT", KEY_VIDEO_NEXT),
                ("VIDEO_PREV", KEY_VIDEO_PREV),
                ("BRIGHTNESS_CYCLE", KEY_BRIGHTNESS_CYCLE),
                ("BRIGHTNESS_AUTO", KEY_BRIGHTNESS_AUTO),
                ("BRIGHTNESS_ZERO", KEY_BRIGHTNESS_ZERO),
                ("BRIGHTNESS_AUTO", KEY_BRIGHTNESS_AUTO),
                ("DISPLAY_OFF", KEY_DISPLAY_OFF),
                ("WWAN", KEY_WWAN),
                ("WIMAX", KEY_WIMAX),
                ("WWAN", KEY_WWAN),
                ("RFKILL", KEY_RFKILL),
                ("MICMUTE", KEY_MICMUTE),

                // below manual shortcuts
                ("PSCR", KEY_SYSRQ),
                ("SLCK", KEY_SCROLLLOCK),
                ("BRK", KEY_PAUSE),
                ("GRV", KEY_GRAVE),
                ("0", KEY_10), // dumb or named wrong?
                ("MINS", KEY_MINUS),
                ("EQL", KEY_EQUAL),
                ("BSPC", KEY_BACKSPACE),
                ("LBRC", KEY_LEFTBRACE),
                ("RBRC", KEY_RIGHTBRACE),
                ("BSLS", KEY_BACKSLASH),
                ("SCLN", KEY_SEMICOLON),
                ("QUOT", KEY_APOSTROPHE),
                ("ENT", KEY_ENTER),
                ("COMM", KEY_COMMA),
                ("DOT", KEY_DOT),
                ("SLSH", KEY_SLASH),

                ("CAPS", KEY_CAPSLOCK),
                ("LSFT", KEY_LEFTSHIFT),
                ("RSFT", KEY_RIGHTSHIFT),
                ("SPC", KEY_SPACE),
                ("APP", KEY_COMPOSE),

                ("LCTL", KEY_LEFTCTRL),
                ("RCTL", KEY_RIGHTCTRL),
                ("LALT", KEY_LEFTALT),
                ("RALT", KEY_RIGHTALT),
                ("LGUI", KEY_LEFTMETA),
                ("RGUI", KEY_RIGHTMETA),

                ("INS", KEY_INSERT),
                ("PGUP", KEY_PAGEUP),
                ("PGDN", KEY_PAGEDOWN),
                ("DEL", KEY_DELETE),

                ("RGHT", KEY_RIGHT),

                ("NLCK", KEY_NUMLOCK),
                ("PSLS", KEY_KPSLASH),
                ("PAST", KEY_KPASTERISK),
                ("PMNS", KEY_KPMINUS),
                ("P7", KEY_KP7),
                ("P8", KEY_KP8),
                ("P9", KEY_KP9),
                ("P4", KEY_KP4),
                ("P5", KEY_KP5),
                ("P6", KEY_KP6),
                ("PPLS", KEY_KPPLUS),
                ("P1", KEY_KP1),
                ("P2", KEY_KP2),
                ("P3", KEY_KP3),
                ("P0", KEY_KP0),
                ("PDOT", KEY_KPDOT),
                ("PENT", KEY_KPENTER),

            ].iter().cloned().map(|(m, v)| (m, v as *const c_int)).collect()
    }

    pub fn new() -> Self {
        //let mut keymap = [0u16; KEY_MAX];
        //let mut keymap : [Box<KeyMapper>; KEY_MAX] = [Box::new(NOOP); KEY_MAX];
        //let mut keymap : [Box<KeyMapper>; KEY_MAX] = [Box::new(0u16); KEY_MAX];
        let mut keymap : Vec<Box<KeyMapper>> = Vec::with_capacity(KEY_MAX);
        for x  in 0..KEY_MAX {
            keymap.push(Box::new(NOOP));
        }
        // which is rustier
        /*
        for x  in 0..KEY_MAX {
            keymap[x as usize] = x as u16;
        }
        for (x, v) in keymap.iter_mut().enumerate() {
            *v = x as u16;
        }
        */
        //println!("keymap: {:?}", &keymap[..]);
        KeyMap {
            keymap: keymap
        }
    }
/*
    pub fn map(&mut self, from : u16, to: u16) {
        self.keymap[from as usize] = to;
    }
*/
    pub fn map(&mut self, from : u16, to: Box<KeyMapper>) {
        self.keymap[from as usize] = to;
    }
}

impl KeyMapper for KeyMap {
    fn send_event(&mut self, key_state: &mut [bool], mut event: input_event, device: &Device) {
        self.keymap[event.code as usize].send_event(key_state, event, device);
        //event.code = self.keymap[event.code as usize];
        //device.write_event(event).expect("could not write event?");
    }
}

impl KeyMapper for u16 {
    fn send_event(&mut self, key_state: &mut [bool], mut event: input_event, device: &Device) {
        event.code = *self;
        device.write_event(event).expect("could not write event?");
    }
}

const NOOP : Noop = Noop{};
// nightly I hear... const BOX_NOOP : Box<KeyMapper> = Box::new(NOOP);
struct Noop {}
impl KeyMapper for Noop {
    fn send_event(&mut self, key_state: &mut [bool], mut event: input_event, device: &Device) {
        device.write_event(event).expect("could not write event?");
    }
}

struct HalfInvertedKey {
    code: u16, // code this is describing
    invert_shift: bool, // true to invert shift for this code
    capslock_nomodify: bool, // true means capslock does not normally modify this, but you would like it to
}

impl HalfInvertedKey {
    fn send_key(&mut self, key_state: &mut [bool], mut event: input_event, device: &Device, left_shift: bool, right_shift: bool, caps_lock: bool) {
        let mut code = self.code;
        let value = event.value;
        let mut invert_shift = self.invert_shift;
        if value == DOWN {
            if caps_lock && self.capslock_nomodify {
                invert_shift = !invert_shift;
            }
            if invert_shift {
                if left_shift {
                    event.code = KEY_LEFTSHIFT_U16;
                    event.value = UP;
                } else if right_shift {
                    event.code = KEY_RIGHTSHIFT_U16;
                    event.value = UP;
                } else {
                    event.code = KEY_LEFTSHIFT_U16;
                    event.value = DOWN;
                }
                //event.code.send_event(key_state, event, device);
                device.write_event(event).expect("could not write event?");
                event.code = code; // not needed since u16 does it
                event.value = value;
            }
        }
        code.send_event(key_state,event, device);
        if value == UP {
            if caps_lock && self.capslock_nomodify {
                invert_shift = !invert_shift;
            }
            if invert_shift {
                if left_shift {
                    event.code = KEY_LEFTSHIFT_U16;
                    event.value = DOWN;
                } else if right_shift {
                    event.code = KEY_RIGHTSHIFT_U16;
                    event.value = DOWN;
                } else {
                    event.code = KEY_LEFTSHIFT_U16;
                    event.value = UP;
                }
                //event.code.send_event(key_state, event, device);
                device.write_event(event).expect("could not write event?");
                // neither of these are needed now...
                event.code = code; // not needed since u16 does it
                event.value = value;
            }
        }
    }
}

impl KeyMapper for HalfInvertedKey {
    fn send_event(&mut self, key_state: &mut [bool], mut event: input_event, device: &Device) {
        let left_shift = key_state[LEFTSHIFT_INDEX];
        let right_shift = key_state[RIGHTSHIFT_INDEX];
        let caps_lock = key_state[CAPSLOCK_INDEX];
        self.send_key(key_state, event, device, left_shift, right_shift, caps_lock);
    }
}

struct ShiftInvertedKey {
    noshift_half: HalfInvertedKey,
    shift_half: HalfInvertedKey,
}

impl KeyMapper for ShiftInvertedKey {
    fn send_event(&mut self, key_state: &mut [bool], mut event: input_event, device: &Device) {
        let left_shift = key_state[LEFTSHIFT_INDEX];
        let right_shift = key_state[RIGHTSHIFT_INDEX];
        let caps_lock = key_state[CAPSLOCK_INDEX];
        if caps_lock != (left_shift || right_shift) {
            self.shift_half.send_key(key_state, event, device, left_shift, right_shift, caps_lock);
        } else{
            self.noshift_half.send_key(key_state, event, device, left_shift, right_shift, caps_lock);
        }
    }
}

#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::path::Path;

#[derive(Deserialize, Debug)]
struct KeymapConfig {
    switch_layout_keys: Vec<String>,
    revert_default_key: String,
    revert_keymap_index: usize,
    default_keymap_index: usize,
    caps_lock_modify: String,
    keymaps: Vec<String>
}

/*
‎c`p‎: no, ? converts the error with From<> in its expansion
‎c`p‎: so unless io::Error: From<toml::Error> it isnt going to work
c`p‎: the idea with your own error type is that MyError: From<io::Error> + From<toml::Error> + etc etc
‎c`p‎: ie enum MyError { Io(io::Error), Toml(toml::Error), ... }
c`p‎: error-chain does all this stuff for you
*/

use std::io::{Error, ErrorKind};

fn parse_cfg<P: AsRef<Path>>(path: P) -> Result<KeymapConfig, Error> {
    let mut f = File::open(path)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    //toml::from_str(&input)?
    match toml::from_str(&input) {
        Ok(toml) => Ok(toml),
        Err(_) => Err(Error::new(ErrorKind::Other, "oh no!"))
    }
}
