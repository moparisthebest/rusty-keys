#![windows_subsystem = "windows"]

use crate::*;
use std::env;
use std::process::exit;

use getopts::Options;
use std::fs::File;

use winapi::um::winuser::{KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, MSG, GetMessageW, CallNextHookEx, SetWindowsHookExW, INPUT_KEYBOARD, MapVirtualKeyW, LPINPUT, INPUT, KEYBDINPUT, SendInput, KEYEVENTF_SCANCODE, KEYEVENTF_KEYUP, WM_KEYUP, WM_KEYDOWN, MAPVK_VK_TO_VSC, ShowWindow, SW_HIDE};
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

use lazy_static::lazy_static;
use std::sync::Mutex;
use winapi::um::wincon::GetConsoleWindow;

type WindowsKeyMaps = KeyMaps<Device, USizeableDWORD, InputEvent, LRESULT>;

// this is used for identifying the fake keypresses we insert, so we don't process them in an infinite loop
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

unsafe impl Send for WindowsKeyMaps {
    // windows promises us keybd_proc will only be called by a single thread at a time
    // but rust makes us wrap in mutex anyway, so we are extra safe...
}

const DEVICE: Device = Device;

lazy_static! {
static ref KEY_MAPPER: Mutex<WindowsKeyMaps> = {

    let config = parse_args();
    //println!("Config: {:?}", config);

    let key_map = key_map();
    //println!("key_map: {:?}", key_map);

    println!("chosen config file: {}", config.config_file);

    Mutex::new(WindowsKeyMaps::from_cfg(&key_map, &config.config_file))
};
}

unsafe extern "system" fn keybd_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let kb_struct = *(l_param as *const KBDLLHOOKSTRUCT);
    //println!("value: {:X}, key: {:X}", w_param, kb_struct.vkCode);
    if kb_struct.dwExtraInfo == FAKE_EXTRA_INFO {
        return CallNextHookEx(null_mut(), code, w_param, l_param);
    }

    /*
    println!("code: {}, w_param: {}, vkCode: {}, scanCode: {}, flags: {}, time: {}, dwExtraInfo: {}",
             code, w_param,
             kb_struct.vkCode, kb_struct.scanCode, kb_struct.flags, kb_struct.time, kb_struct.dwExtraInfo);
    */

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
    // this is just to cause the lazy_static init to run first, so if -h or -v is wanted, we do that
    // and exit immediately... todo: how to avoid mutex/lazy_static entirely???
    let _ = KEY_MAPPER.lock().unwrap();

    // now start actually intercepting keypresses
    let keybd_hhook: AtomicPtr<HHOOK__> = AtomicPtr::default();
    set_hook(WH_KEYBOARD_LL, &keybd_hhook, keybd_proc);

    println!("rusty-keys {} keyboard hook registered, now for some reason you *sometimes* have to type in this window once to activate it, thanks windows!", VERSION);

    unsafe {
        // hide window
        // todo: probably should be tray icon someplace in future to quit, and error messages as windows popups etc...
        let hwnd = GetConsoleWindow();
        ShowWindow( hwnd, SW_HIDE );
    }
    
    let mut msg: MSG = unsafe { zeroed() };
    unsafe { GetMessageW(&mut msg, 0 as HWND, 0, 0) };
    
    //std::thread::sleep(std::time::Duration::from_millis(400000));
    
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
