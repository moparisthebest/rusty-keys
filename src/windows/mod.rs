
use crate::*;
use std::env;
use std::process::exit;

use getopts::Options;

use winapi::um::winuser::{KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, MSG, GetMessageW, CallNextHookEx, SetWindowsHookExW, INPUT_KEYBOARD, MapVirtualKeyW, LPINPUT, INPUT, KEYBDINPUT, SendInput, KEYEVENTF_SCANCODE, KEYEVENTF_KEYUP, WM_KEYUP, WM_KEYDOWN, MAPVK_VK_TO_VSC};
use winapi::shared::windef::{HHOOK__, HWND};
use winapi::shared::minwindef::{LRESULT, WPARAM, LPARAM, HINSTANCE};
use winapi::shared::basetsd::ULONG_PTR;
use winapi::ctypes::c_int;

pub mod codes;
use codes::*;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::mem::{zeroed, size_of};
use winapi::_core::ptr::null_mut;
use winapi::_core::mem::transmute_copy;

type WindowsKeyMaps = KeyMaps<Device, USizeableDWORD, InputEvent, LRESULT>;

const FAKE_EXTRA_INFO: ULONG_PTR = 332;

// non-zero means don't send on, I think https://msdn.microsoft.com/en-us/library/windows/desktop/ms644984(v=vs.85).aspx
const BLOCK_KEY: LRESULT = 1;

struct InputEvent {
    code: c_int,
    value: WPARAM,
    kb_hook_pointer: LPARAM,
    kb_hook_struct: KBDLLHOOKSTRUCT,
}

impl KeyEvent<USizeableDWORD> for InputEvent {
    fn code(&self) -> USizeableDWORD {
        USizeableDWORD(self.kb_hook_struct.vkCode)
    }

    fn value(&self) -> KeyState {
        match self.value as u32 {
            WM_KEYUP => KeyState::UP,
            WM_KEYDOWN => KeyState::DOWN,
            _ => KeyState::OTHER,
        }
    }
}

struct Device;

impl Keyboard<USizeableDWORD, InputEvent, LRESULT> for Device {
    fn send(&self, event: &mut InputEvent) -> Result<LRESULT> {
        Ok(unsafe { CallNextHookEx(null_mut(), event.code, event.value, event.kb_hook_pointer) })
    }

    fn send_mod_code(&self, code: USizeableDWORD, event: &mut InputEvent) -> Result<LRESULT> {
        // event.value should only ever be UP/DOWN when this method is called
        self.send_mod_code_value(code, event.value as u32 == WM_KEYUP, event)
    }

    fn send_mod_code_value(&self, code: USizeableDWORD, up_not_down: bool, _event: &mut InputEvent) -> Result<LRESULT> {
        let flags = if up_not_down {
            KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP
        } else {
            KEYEVENTF_SCANCODE
        };
        send_keybd_input(flags, code);
        Ok(BLOCK_KEY)
    }

    fn synchronize(&self) -> Result<LRESULT> {
        // no-op here
        Ok(0)
    }

    fn left_shift_code(&self) -> USizeableDWORD {
        USizeableDWORD(KEY_LEFTSHIFT)
    }

    fn right_shift_code(&self) -> USizeableDWORD {
        USizeableDWORD(KEY_RIGHTSHIFT)
    }

    fn caps_lock_code(&self) -> USizeableDWORD {
        USizeableDWORD(KEY_CAPSLOCK)
    }

    fn block_key(&self) -> Result<LRESULT> {
        Ok(BLOCK_KEY)
    }
}

unsafe impl Sync for WindowsKeyMaps {
    // this isn't safe, but windows promises us keybd_proc will only be called by a single thread at
    // a time, so if that holds true, this is safe
}

unsafe impl Send for WindowsKeyMaps {

}

const DEVICE: Device = Device;

//static mut KEY_MAPPER_PTR: *const Option<WindowsKeyMaps> = &None;
//static mut KEY_MAPPERe: WindowsKeyMaps = unsafe { std::mem::uninitialized() };
//static mut KEY_MAPPER: Option<WindowsKeyMaps> = None;
//const KEY_MAPPER_PTR: i32 = 0;
/*
*/
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
static ref KEY_MAPPER: Mutex<WindowsKeyMaps> = {

    let config = parse_args();
    println!("Config: {:?}", config);

    let key_map = key_map();
    //println!("key_map: {:?}", key_map);

    println!("config_file: {}", config.config_file);

    Mutex::new(WindowsKeyMaps::from_cfg(&key_map, &config.config_file))
};
}

unsafe extern "system" fn keybd_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let kb_struct = *(l_param as *const KBDLLHOOKSTRUCT);
    println!("value: {:X}, key: {:X}", w_param, kb_struct.vkCode);
    if kb_struct.dwExtraInfo == FAKE_EXTRA_INFO {
        return CallNextHookEx(null_mut(), code, w_param, l_param);
    }

    println!("code: {}, w_param: {}, vkCode: {}, scanCode: {}, flags: {}, time: {}, dwExtraInfo: {}",
             code, w_param,
             kb_struct.vkCode, kb_struct.scanCode, kb_struct.flags, kb_struct.time, kb_struct.dwExtraInfo);

    let mut input_event = InputEvent{
        code,
        value: w_param,
        kb_hook_pointer: l_param,
        kb_hook_struct: kb_struct,
    };

    // .unwrap() is ok because windows impl can actually never can fail
    //DEVICE.send(&mut input_event).unwrap()
    //KEY_MAPPER.send_event(&mut input_event, &DEVICE).unwrap()
    KEY_MAPPER.lock().unwrap().send_event(&mut input_event, &DEVICE).unwrap()
}

fn set_hook(
    hook_id: i32,
    hook_ptr: &AtomicPtr<HHOOK__>,
    hook_proc: unsafe extern "system" fn (c_int, WPARAM, LPARAM) -> LRESULT,
) {
    hook_ptr.store(
        unsafe { SetWindowsHookExW(hook_id, Some(hook_proc), 0 as HINSTANCE, 0) },
        Ordering::Relaxed,
    );
}

fn send_keybd_input(flags: u32, key_code: USizeableDWORD) {
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe {
            transmute_copy(&KEYBDINPUT {
                wVk: 0,
                wScan: MapVirtualKeyW(key_code.0, MAPVK_VK_TO_VSC) as u16,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: FAKE_EXTRA_INFO,
            })
        },
    };
    unsafe { SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int) };
}


pub fn main_res() -> Result<()> {
    /*
    let config = parse_args();
    println!("Config: {:?}", config);

    let key_map = key_map();
    //println!("key_map: {:?}", key_map);

    println!("caps_lock_code: {}", DEVICE.caps_lock_code().0);

    let key_map = WindowsKeyMaps::from_cfg(&key_map, &config.config_file);
    */
    //let mut ptr: *const WindowsKeyMaps = &key_map;
    //let key_map = Box::new(key_map);
    //let static_ref = Box::leak(key_map);
    //let raw = Box::into_raw(key_map);
    //let ptr: *const i32 = &raw;
    //let my_num_ptr: *const i32 = &*key_map;
    //key_map.leak();
    //KEY_MAPPER_PTR = Box::into_raw(key_map);
    //unsafe { KEY_MAPPER = Some(key_map); }
    //unsafe { KEY_MAPPER_PTR = &Some(key_map); }
    //let bla = unsafe {*KEY_MAPPER_PTR};

    // now start actually intercepting keypresses
    let keybd_hhook: AtomicPtr<HHOOK__> = AtomicPtr::default();
    set_hook(WH_KEYBOARD_LL, &keybd_hhook, keybd_proc);
    let mut msg: MSG = unsafe { zeroed() };
    unsafe { GetMessageW(&mut msg, 0 as HWND, 0, 0) };

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

fn parse_args() -> Config {
    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} [options]", program);
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

    Config::new(config_file)
}
