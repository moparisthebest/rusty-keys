
use crate::*;

use crate::linux::device::codes::*;

pub mod device;
pub use device::{Device,InputDevice, Builder};

use libc::input_event;
use std::process::exit;
use std::env;
use std::collections::HashMap;

#[cfg(feature = "epoll_inotify")]
const INPUT_FOLDER: &str = "/dev/input/";

// 1 is down, 0 is up
const DOWN: i32 = 1;
const UP: i32 = 0;

use getopts::Options;

const EV_KEY_U16: u16 = EV_KEY as u16;

type LinuxKeyMaps = KeyMaps<Device, u16, input_event>;

impl KeyEvent<u16> for input_event {
    fn code(&self) -> u16 {
        self.code
    }

    fn value(&self) -> KeyState {
        match self.value {
            UP => KeyState::UP,
            DOWN => KeyState::DOWN,
            _ => KeyState::OTHER,
        }
    }
}

impl Keyboard<u16, input_event> for Device {
    fn send(&self, event: &mut input_event) -> Result<()> {
        self.write_event(event)
    }

    fn send_mod_code(&self, code: u16, event: &mut input_event) -> Result<()> {
        event.code = code;
        Keyboard::send(self, event)
    }

    fn send_mod_code_value(&self, code: u16, up_not_down: bool, event: &mut input_event) -> Result<()> {
        event.code = code;
        let value = event.value;
        if up_not_down {
            event.value = UP;
        } else {
            event.value = DOWN;
        }
        Keyboard::send(self, event)?;
        // set it back
        event.value = value;
        Ok(())
    }

    fn synchronize(&self) -> Result<()> {
        Device::synchronize(self)
    }

    fn left_shift_code(&self) -> u16 {
        KEY_LEFTSHIFT as u16
    }

    fn right_shift_code(&self) -> u16 {
        KEY_RIGHTSHIFT as u16
    }

    fn caps_lock_code(&self) -> u16 {
        KEY_CAPSLOCK as u16
    }

    fn block_key(&self) -> Result<()> {
        Ok(()) // we don't actually use/need this here
    }
}

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

pub fn main_res() -> Result<()> {
    let config = parse_args();
    //println!("Config: {:?}", config);

    let key_map = key_map();
    //println!("key_map: {:?}", key_map);

    let device = Builder::open("/dev/uinput")
        .or_else(|_| Builder::open("/dev/input/uinput"))
        .or_else(|_| Builder::default())?
        .name(NAME)?
        .event(key_map.values())?
        .create()?;

    #[cfg(not(feature = "toml_serde"))]
    let mut key_map = LinuxKeyMaps::new(&key_map, KeymapConfig::default());
    #[cfg(feature = "toml_serde")]
    let mut key_map = LinuxKeyMaps::from_cfg(&key_map, &config.config_file);
    //println!("keymaps: {:?}", keymaps);

    let mut input_event_buf = InputDevice::new_input_event_buf();

    if config.device_files.len() == 1 {
        // shortcut, don't bother with epoll
        let mut input_device = InputDevice::open(&config.device_files[0])?.grab()?;

        loop {
            let event = input_device.read_event(&mut input_event_buf)?;
            send_event(&mut key_map, event, &device)?
        }
    } else {
        #[cfg(not(feature = "epoll_inotify"))]
        panic!("without epoll_inotify feature, only exactly 1 device is supported");

        #[cfg(feature = "epoll_inotify")]
            {
                use inotify::{Inotify, WatchMask};

                let epoll_fd = epoll::create(true)?;
                const INOTIFY_DATA: u64 = u64::max_value();

                let (device_files, mut inotify) = if config.device_files.len() > 0 {
                    // we operate on exactly the devices sent in and never watch for new devices
                    (config.device_files.iter().map(|device_file| InputDevice::open(&device_file).expect("device_file does not exist!")).collect(), None)
                } else {
                    use std::os::unix::io::AsRawFd;
                    // we want to wait forever starting new threads for any new keyboard devices
                    // there is a slight race condition here, if a keyboard is plugged in between the time we
                    // enumerate the devices and set up the inotify watch, we'll miss it, doing it the other way
                    // can bring duplicates though, todo: think about this...
                    let device_files = get_keyboard_devices();
                    let mut inotify = Inotify::init()?;
                    inotify.add_watch(INPUT_FOLDER, WatchMask::CREATE)?;
                    let epoll_event = epoll::Event::new(epoll::Events::EPOLLIN | epoll::Events::EPOLLET, INOTIFY_DATA);
                    epoll::ctl(epoll_fd, epoll::ControlOptions::EPOLL_CTL_ADD, inotify.as_raw_fd(), epoll_event)?;
                    (device_files, Some(inotify))
                };
                let mut input_devices = Vec::with_capacity(device_files.len());
                for (idx, device_file) in device_files.into_iter().enumerate() {
                    input_devices.push(Some(device_file.grab()?.epoll_add(epoll_fd, idx as u64)?));
                }

                let mut epoll_buf = [epoll::Event::new(epoll::Events::empty(), 0); 4];
                let mut inotify_buf = [0u8; 256];

                loop {
                    let num_events = epoll::wait(epoll_fd, -1, &mut epoll_buf)?;
                    for event in &epoll_buf[0..num_events] {
                        let idx = event.data as usize;
                        if let Some(Some(input_device)) = &mut input_devices.get_mut(idx) {
                            loop {
                                match input_device.read_event(&mut input_event_buf) {
                                    Ok(event) => {
                                        //println!("input event: {:?}", event);
                                        send_event(&mut key_map, event, &device)?
                                    }
                                    Err(err) => {
                                        if let Error::Io(ref err) = err {
                                            if err.kind() == std::io::ErrorKind::WouldBlock {
                                                // go back to epoll event loop
                                                break;
                                            }
                                        }
                                        // otherwise it's some other error, don't read anything from this again
                                        println!("input err: {:?}", err);
                                        // remove it from input_devices and drop it
                                        let _ = std::mem::replace(&mut input_devices[idx], None);
                                        if inotify.is_none() {
                                            // if we aren't watching with inotify, and the last device is removed (Vec only has None's in it), exit the program
                                            if input_devices.iter().all(|id| id.is_none()) {
                                                println!("last device went away, exiting...");
                                                return Ok(());
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                        } else if event.data == INOTIFY_DATA {
                            if let Some(inotify) = &mut inotify {
                                for event in inotify.read_events(&mut inotify_buf)? {
                                    if let Some(device_file) = event.name.and_then(|name| name.to_str()) {
                                        // check if this is an eligible keyboard device
                                        let mut path = std::path::PathBuf::new();
                                        path.push(INPUT_FOLDER);
                                        path.push(device_file);

                                        if let Ok(input_device) = InputDevice::open(path).and_then(|id| id.valid_keyboard_device()) {
                                            println!("starting mapping for new keyboard: {}", device_file);

                                            let idx = input_devices.iter().position(|id| id.is_none()).unwrap_or(input_devices.len());

                                            let input_device = input_device.grab()?.epoll_add(epoll_fd, idx as u64)?;

                                            if idx == input_devices.len() {
                                                input_devices.push(Some(input_device));
                                            } else {
                                                // simply replacing None here
                                                let _ = std::mem::replace(&mut input_devices[idx], Some(input_device));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }
}

fn send_event(key_map: &mut LinuxKeyMaps, mut event: input_event, device: &Device) -> Result<()> {
    if event.type_ == EV_KEY_U16 {
        // println!("type: {} code: {:?} value: {:?}", event.type_, event.code(), event.value());
        key_map.send_event(&mut event, &device)?
    } else {
        device.write_event(&mut event)?
    }
    Ok(())
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
        println!("{} {}", NAME, VERSION);
        exit(0);
    }

    let config_file = matches.opt_str("c").unwrap_or("/etc/rusty-keys/keymap.toml".to_owned());

    Config::new(matches.free, config_file)
}

#[cfg(feature = "epoll_inotify")]
fn get_keyboard_devices() -> Vec<InputDevice> {
    let mut res = Vec::new();
    if let Ok(entries) = std::fs::read_dir(INPUT_FOLDER) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(input_device) = InputDevice::open(entry.path()).and_then(|id|id.valid_keyboard_device()) {
                    res.push(input_device);
                }
            }
        }
    }
    res
}

pub fn key_map() -> HashMap<&'static str, u16> {
        [
            // generated like:
            // grep -o 'KEY_[^ :;]*' ~/.cargo/registry/src/github.com-1ecc6299db9ec823/uinput-sys-0.1.3/src/codes | sed 's/^KEY_//' | awk '{print "(\""$1"\", KEY_"$1"),"}'
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
            ("TOUCHPAD_TOGGLE", KEY_TOUCHPAD_TOGGLE),
        ].iter().cloned().map(|(m, v)| (m, v as u16)).collect()
    }

