#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use crate::*;
use std::env;
use std::process::exit;

use getopts::Options;
use std::fs::File;

pub mod codes;
use codes::*;

use core_graphics::event::CGKeyCode;
use core_graphics::event::*;
use core_graphics::event_source::*;

use core_graphics::event::{CGEventTapLocation, CGEventType};

type MacOSKeyMaps = KeyMaps<CGEventSource, CGKeyCode, CGEvent, Option<CGEvent>>;
type CallbackPointer = (MacOSKeyMaps, CGEventSource);

/*
// possible types for event_source
Private = -1,
CombinedSessionState = 0,
HIDSystemState = 1,

// possible types for tapLocation
HID,
Session,
AnnotatedSession,
 */

// macOS seems to require this, or it ignores shift, WHY?
const delay: std::time::Duration = std::time::Duration::from_millis(20);
const tapLocation: CGEventTapLocation = CGEventTapLocation::Session;
// this is only used if tapLocation is HID, to prevent us from mapping our own key inputs
const uniqueHIDUserData: i64 = 45;

impl KeyEvent<CGKeyCode> for CGEvent {
    fn code(&self) -> CGKeyCode {
        self.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE) as CGKeyCode
    }

    fn value(&self) -> KeyState {
        let event_type = self.get_type();
        match event_type {
            CGEventType::FlagsChanged => {
                let flags = self.get_flags().bits(); // todo: fix cast?
                let mask = match self.code() {
                    KEY_LEFTCTRL => NX_DEVICELCTLKEYMASK,
                    KEY_RIGHTCTRL => NX_DEVICERCTLKEYMASK,
                    KEY_LEFTSHIFT => NX_DEVICELSHIFTKEYMASK,
                    KEY_RIGHTSHIFT => NX_DEVICERSHIFTKEYMASK,
                    KEY_LEFTMETA => NX_DEVICELCMDKEYMASK,
                    KEY_RIGHTMETA => NX_DEVICERCMDKEYMASK,
                    KEY_LEFTALT => NX_DEVICELALTKEYMASK,
                    KEY_RIGHTALT => NX_DEVICERALTKEYMASK,
                    KEY_CAPSLOCK => NX_DEVICECAPSLOCKMASK,
                    _ => panic!("unhandled key: {}", self.code()),
                };
                if (flags & mask) != 0 { KeyState::DOWN } else { KeyState::UP }
            },
            CGEventType::KeyDown => KeyState::DOWN,
            CGEventType::KeyUp => KeyState::UP,
            CGEventType::TapDisabledByTimeout => {
                println!("Quartz event tap disabled because of timeout; attempting to reregister.");
                todo!("implement register listener");
                //register_listener(channel);
                //KeyState::OTHER
            },
            _ => {
                println!("Received unknown EventType: {:?}", event_type);
                KeyState::OTHER
            },
        }
    }
}

impl Keyboard<CGKeyCode, CGEvent, Option<CGEvent>> for CGEventSource {
    fn send(&self, event: &mut CGEvent) -> Result<Option<CGEvent>> {
        //println!("send orig: {}", event.code());

        //Ok(Some(event.event))

        self.send_mod_code_value(event.code(), event.value() == KeyState::UP, event)
    }

    fn send_mod_code(&self, code: CGKeyCode, event: &mut CGEvent) -> Result<Option<CGEvent>> {
        // event.value should only ever be UP/DOWN when this method is called
        //println!("send_mod_code orig: {} code: {}", event.code(), code);

        //unsafe { CGEventSetIntegerValueField(event.event, kCGKeyboardEventKeycode, code as i64) };
        //Ok(Some(event.event))

        self.send_mod_code_value(code, event.value() == KeyState::UP, event)
    }

    fn send_mod_code_value(&self, code: CGKeyCode, up_not_down: bool, _event: &mut CGEvent) -> Result<Option<CGEvent>> {
        //println!("send_mod_code_value orig: {} code: {}, up_not_down: {}", event.code(), code, up_not_down);
        //return Ok(None);

        let event =
            CGEvent::new_keyboard_event(self.clone(), code, !up_not_down)
                .expect("Failed creating event");
        match tapLocation {
            CGEventTapLocation::HID => event.set_integer_value_field(EventField::EVENT_SOURCE_USER_DATA, uniqueHIDUserData),
            _ => {}
        };
        event.post(tapLocation);

        Ok(None)
    }

    fn synchronize(&self) -> Result<Option<CGEvent>> {
        std::thread::sleep(delay);
        Ok(None)
    }

    fn left_shift_code(&self) -> CGKeyCode {
        KEY_LEFTSHIFT
    }

    fn right_shift_code(&self) -> CGKeyCode {
        KEY_RIGHTSHIFT
    }

    fn caps_lock_code(&self) -> CGKeyCode {
        KEY_CAPSLOCK
    }

    fn block_key(&self) -> Result<Option<CGEvent>> {
        Ok(None)
    }
}

pub fn main_res() -> Result<()> {
    let config = parse_args();
    //println!("Config: {:?}", config);

    let key_map = key_map();
    //println!("key_map: {:?}", key_map);

    let key_maps = MacOSKeyMaps::from_cfg(&key_map, &config.config_file);
    //println!("key_maps: {}", key_maps);
    let callback_pointer: CallbackPointer = (key_maps, CGEventSource::new(CGEventSourceStateID::Private).expect("Failed creating event source"));

    let mask = CGEventMaskBit(CGEventType::KeyDown)
        | CGEventMaskBit(CGEventType::KeyUp)
        | CGEventMaskBit(CGEventType::FlagsChanged)
        ;

    unsafe {
        let options = 0;

        // Create the event tap
        let event_tap = CGEventTapCreate(
            kCGSessionEventTap,
            kCGHeadInsertEventTap,
            options,
            mask,
            callback,
            &callback_pointer,
        );
        if event_tap.is_null() {
            panic!("Unable to create event tap. Please make sure you have the correct permissions");
        }
        println!("Created event tap...");

        let allocator = kCFAllocatorDefault;
        let current_event_loop = CFRunLoopGetCurrent();
        let mode = kCFRunLoopCommonModes;

        // Create Run Loop Source
        let run_loop_source = CFMachPortCreateRunLoopSource(allocator, event_tap, 0);

        // Add Run Loop Source to the current event loop
        CFRunLoopAddSource(current_event_loop, run_loop_source, mode);

        // Enable the tap
        CGEventTapEnable(event_tap, true);

        CFRunLoopRun();
    }

    Ok(())
}

#[derive(Debug)]
struct Config {
    config_file: String
}

impl Config {
    fn new(config_file: String) -> Self {
        Config { config_file: config_file }
    }
}

fn get_env_push(key: &str, to_push: &str, vec: &mut Vec<String>) {
    if let Some(var) = env::var_os(key) {
        if let Ok(str) = var.into_string() {
            let mut str = str.to_owned();
            str.push_str(to_push);
            vec.push(str);
        }
    }
}

fn parse_args() -> Config {
    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} [options] [keymap.toml]", program);
        println!("{}", opts.usage(&brief));
    }

    let args: Vec<_> = env::args().collect();

    let mut default_configs = Vec::new();
    get_env_push("USERPROFILE", "\\keymap.toml", &mut default_configs);
    get_env_push("APPDATA", "\\keymap.toml", &mut default_configs);

    default_configs.push("keymap.toml".to_string());

    let c_msg = format!("specify the keymap config file to use (default in order: {:?})", default_configs);

    let mut opts = Options::new();
    opts.optflag("h", "help", "prints this help message");
    opts.optflag("v", "version", "prints the version");
    opts.optopt("c", "config", &c_msg, "FILE");

    let matches = opts.parse(&args[1..]);
    if matches.is_err() {
        print_usage(&args[0], opts);
        exit(1);
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

    let config_file = matches.opt_str("c").unwrap_or_else(|| {
        let remaining_args = matches.free;
        if remaining_args.len() > 0 {
            remaining_args[0].clone()
        } else {
            for keymap in default_configs.drain(..) {
                if File::open(&keymap).is_ok() {
                    return keymap;
                }
            }
            println!("Error: no keymap.toml found...");
            print_usage(&args[0], opts);
            exit(1);
        }
    });

    Config::new(config_file)
}

use libc;

// Opaque Pointer Types
type Pointer = *mut libc::c_void;
type CFMachPortRef = Pointer;

// Integer Types
type CGEventMask = u64;
type CGEventTapOptions = u32;
type CGEventTapPlacement = u32;

// Callback Type
type CGEventTapCallBack = extern "C" fn(Pointer, CGEventType, CGEvent, &mut CallbackPointer) -> CGEvent;

// Constants
const kCGSessionEventTap: CGEventTapLocation = CGEventTapLocation::HID;
const kCGHeadInsertEventTap: CGEventTapPlacement = 0;

    // Link to ApplicationServices/ApplicationServices.h and Carbon/Carbon.h
    #[link(name = "ApplicationServices", kind = "framework")]
    #[link(name = "Carbon", kind = "framework")]
    extern {

        /// Pass through to the default loop modes
        pub static kCFRunLoopCommonModes: Pointer;

        /// Pass through to the default allocator
        pub static kCFAllocatorDefault: Pointer;

        /// Run the current threads loop in default mode
        pub fn CFRunLoopRun();

        /// Obtain the current threads loop
        pub fn CFRunLoopGetCurrent() -> Pointer;

        /// Create an event tap
        ///
        /// # Arguments
        ///
        /// * `place` - The location of the new event tap. Pass one of
        ///          the constants listed in Event Tap Locations. Only
        ///          processes running as the root user may locate an
        ///          event tap at the point where HID events enter the
        ///          window server; for other users, this function
        ///          returns NULL.
        ///
        /// * `options` - The placement of the new event tap in the
        ///          list of active event taps. Pass one of the
        ///          constants listed in Event Tap Placement.
        ///
        /// * `eventsOfInterest` - A constant that specifies whether
        ///          the new event tap is a passive listener or an
        ///          active filter.
        ///
        /// * `callback` - A bit mask that specifies the set of events
        ///          to be observed. For a list of possible events,
        ///          see Event Types. For information on how to
        ///          specify the mask, see CGEventMask. If the event
        ///          tap is not permitted to monitor one or more of
        ///          the events specified in the eventsOfInterest
        ///          parameter, then the appropriate bits in the mask
        ///          are cleared. If that action results in an empty
        ///          mask, this function returns NULL.  callback
        ///
        /// * `refcon` - An event tap callback function that you
        ///          provide. Your callback function is invoked from
        ///          the run loop to which the event tap is added as a
        ///          source. The thread safety of the callback is
        ///          defined by the run loop’s environment. To learn
        ///          more about event tap callbacks, see
        ///          CGEventTapCallBack.  refcon
        ///
        /// * `channel` - A pointer to user-defined data. This pointer
        ///          is passed into the callback function specified in
        ///          the callback parameter.  Here we use it as a mpsc
        ///          channel.
        pub fn CGEventTapCreate(
            tap: CGEventTapLocation,
            place: CGEventTapPlacement,
            options: CGEventTapOptions,
            eventsOfInterest: CGEventMask,
            callback: CGEventTapCallBack,
            channel: &CallbackPointer,
        ) -> CFMachPortRef;

        /// Creates a CFRunLoopSource object for a CFMachPort
        /// object.
        ///
        /// The run loop source is not automatically added to
        /// a run loop. To add the source to a run loop, use
        /// CFRunLoopAddSource
        pub fn CFMachPortCreateRunLoopSource(
            allocator: Pointer,
            port: CFMachPortRef,
            order: libc::c_int,
        ) -> Pointer;

        /// Adds a CFRunLoopSource object to a run loop mode.
        pub fn CFRunLoopAddSource(
            run_loop: Pointer,
            run_loop_source: Pointer,
            mode: Pointer,
        );

        pub fn CGEventTapEnable(port: CFMachPortRef, enable: bool);
    }

const NX_DEVICELCTLKEYMASK: u64 =    0x00000001;
const NX_DEVICERCTLKEYMASK: u64 = 0x00002000;
const NX_DEVICELSHIFTKEYMASK: u64 =  0x00000002;
const NX_DEVICERSHIFTKEYMASK: u64 =  0x00000004;
const NX_DEVICELCMDKEYMASK: u64 =    0x00000008;
const NX_DEVICERCMDKEYMASK: u64 =    0x00000010;
const NX_DEVICELALTKEYMASK: u64 =    0x00000020;
const NX_DEVICERALTKEYMASK: u64 =    0x00000040;

const NX_DEVICECAPSLOCKMASK: u64 =    1 << 16;

///  This callback will be registered to be invoked from the run loop
///  to which the event tap is added as a source.
#[no_mangle]
#[allow(unused_variables, improper_ctypes_definitions)]
pub extern fn callback(proxy: Pointer, event_type: CGEventType, mut event: CGEvent, callback_pointer: &mut CallbackPointer) -> CGEvent {
    let (key_maps, event_source) = callback_pointer;
    match tapLocation {
        CGEventTapLocation::HID => {
            let user_data = event.get_integer_value_field(EventField::EVENT_SOURCE_USER_DATA);
            if user_data == uniqueHIDUserData {
                return event;
            }
        }
        _ => {}
    };

    key_maps.send_event(&mut event, &event_source).expect("macos shouldn't error...")
        .unwrap_or_else(|| {
            event.set_type(CGEventType::Null);
            event
        }) // None means return NULL
    }

    /// Redefine macro for bitshifting from header as function here
    pub fn CGEventMaskBit(eventType: CGEventType) -> CGEventMask {
        1 << (eventType as u32)
    }

