use std::{collections::HashMap, convert::TryFrom};
use winapi::shared::minwindef::DWORD;

// https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes

pub const KEY_BACKSPACE: DWORD = 0x08;
pub const KEY_TAB: DWORD = 0x09;
pub const KEY_ENTER: DWORD = 0x0D;
pub const KEY_KPENTER: DWORD = 0x0D; // on windows, same as KEY_ENTER..
pub const KEY_ESC: DWORD = 0x1B;
pub const KEY_SPACE: DWORD = 0x20;
pub const KEY_HOME: DWORD = 0x24;
pub const KEY_LEFT: DWORD = 0x25;
pub const KEY_UP: DWORD = 0x26;
pub const KEY_RIGHT: DWORD = 0x27;
pub const KEY_DOWN: DWORD = 0x28;
pub const KEY_INSERT: DWORD = 0x2D;
pub const KEY_DELETE: DWORD = 0x2E;
pub const KEY_10: DWORD = 0x30; // named 10 on linux, 0 on windows..
pub const KEY_0: DWORD = 0x30;
pub const KEY_1: DWORD = 0x31;
pub const KEY_2: DWORD = 0x32;
pub const KEY_3: DWORD = 0x33;
pub const KEY_4: DWORD = 0x34;
pub const KEY_5: DWORD = 0x35;
pub const KEY_6: DWORD = 0x36;
pub const KEY_7: DWORD = 0x37;
pub const KEY_8: DWORD = 0x38;
pub const KEY_9: DWORD = 0x39;
pub const KEY_A: DWORD = 0x41;
pub const KEY_B: DWORD = 0x42;
pub const KEY_C: DWORD = 0x43;
pub const KEY_D: DWORD = 0x44;
pub const KEY_E: DWORD = 0x45;
pub const KEY_F: DWORD = 0x46;
pub const KEY_G: DWORD = 0x47;
pub const KEY_H: DWORD = 0x48;
pub const KEY_I: DWORD = 0x49;
pub const KEY_J: DWORD = 0x4A;
pub const KEY_K: DWORD = 0x4B;
pub const KEY_L: DWORD = 0x4C;
pub const KEY_M: DWORD = 0x4D;
pub const KEY_N: DWORD = 0x4E;
pub const KEY_O: DWORD = 0x4F;
pub const KEY_P: DWORD = 0x50;
pub const KEY_Q: DWORD = 0x51;
pub const KEY_R: DWORD = 0x52;
pub const KEY_S: DWORD = 0x53;
pub const KEY_T: DWORD = 0x54;
pub const KEY_U: DWORD = 0x55;
pub const KEY_V: DWORD = 0x56;
pub const KEY_W: DWORD = 0x57;
pub const KEY_X: DWORD = 0x58;
pub const KEY_Y: DWORD = 0x59;
pub const KEY_Z: DWORD = 0x5A;
pub const KEY_KP0: DWORD = 0x60;
pub const KEY_KP1: DWORD = 0x61;
pub const KEY_KP2: DWORD = 0x62;
pub const KEY_KP3: DWORD = 0x63;
pub const KEY_KP4: DWORD = 0x64;
pub const KEY_KP5: DWORD = 0x65;
pub const KEY_KP6: DWORD = 0x66;
pub const KEY_KP7: DWORD = 0x67;
pub const KEY_KP8: DWORD = 0x68;
pub const KEY_KP9: DWORD = 0x69;
pub const KEY_F1: DWORD = 0x70;
pub const KEY_F2: DWORD = 0x71;
pub const KEY_F3: DWORD = 0x72;
pub const KEY_F4: DWORD = 0x73;
pub const KEY_F5: DWORD = 0x74;
pub const KEY_F6: DWORD = 0x75;
pub const KEY_F7: DWORD = 0x76;
pub const KEY_F8: DWORD = 0x77;
pub const KEY_F9: DWORD = 0x78;
pub const KEY_F10: DWORD = 0x79;
pub const KEY_F11: DWORD = 0x7A;
pub const KEY_F12: DWORD = 0x7B;
pub const KEY_NUMLOCK: DWORD = 0x90;
pub const KEY_SCROLLLOCK: DWORD = 0x91;
pub const KEY_CAPSLOCK: DWORD = 0x14;
pub const KEY_LEFTSHIFT: DWORD = 0xA0;
pub const KEY_RIGHTSHIFT: DWORD = 0xA1;
pub const KEY_LEFTCTRL: DWORD = 0xA2;
pub const KEY_RIGHTCTRL: DWORD = 0xA3;

pub const KEY_LBUTTON: DWORD = 0x01;
pub const KEY_RBUTTON: DWORD = 0x02;
pub const KEY_CANCEL: DWORD = 0x03;
pub const KEY_MBUTTON: DWORD = 0x04;
pub const KEY_XBUTTON1: DWORD = 0x05;
pub const KEY_XBUTTON2: DWORD = 0x06;
pub const KEY_BACK: DWORD = 0x08;
pub const KEY_CLEAR: DWORD = 0x0C;
pub const KEY_RETURN: DWORD = 0x0D;
pub const KEY_SHIFT: DWORD = 0x10;
pub const KEY_CONTROL: DWORD = 0x11;
pub const KEY_MENU: DWORD = 0x12;
pub const KEY_PAUSE: DWORD = 0x13;
pub const KEY_CAPITAL: DWORD = 0x14;
pub const KEY_KANA: DWORD = 0x15;
pub const KEY_HANGEUL: DWORD = 0x15;
pub const KEY_HANGUL: DWORD = 0x15;
pub const KEY_JUNJA: DWORD = 0x17;
pub const KEY_FINAL: DWORD = 0x18;
pub const KEY_HANJA: DWORD = 0x19;
pub const KEY_KANJI: DWORD = 0x19;
pub const KEY_ESCAPE: DWORD = 0x1B;
pub const KEY_CONVERT: DWORD = 0x1C;
pub const KEY_NONCONVERT: DWORD = 0x1D;
pub const KEY_ACCEPT: DWORD = 0x1E;
pub const KEY_MODECHANGE: DWORD = 0x1F;
pub const KEY_PAGEUP: DWORD = 0x21;
pub const KEY_PAGEDOWN: DWORD = 0x22;
pub const KEY_END: DWORD = 0x23;
pub const KEY_SELECT: DWORD = 0x29;
pub const KEY_PRINT: DWORD = 0x2A;
pub const KEY_EXECUTE: DWORD = 0x2B;
pub const KEY_SYSRQ: DWORD = 0x2C;
pub const KEY_HELP: DWORD = 0x2F;
pub const KEY_LEFTMETA: DWORD = 0x5B;
pub const KEY_RIGHTMETA: DWORD = 0x5C;
pub const KEY_COMPOSE: DWORD = 0x5D;
pub const KEY_SLEEP: DWORD = 0x5F;
pub const KEY_KPASTERISK: DWORD = 0x6A;
pub const KEY_KPPLUS: DWORD = 0x6B;
pub const KEY_SEPARATOR: DWORD = 0x6C;
pub const KEY_KPMINUS: DWORD = 0x6D;
pub const KEY_KPDOT: DWORD = 0x6E;
pub const KEY_KPSLASH: DWORD = 0x6F;
pub const KEY_F13: DWORD = 0x7C;
pub const KEY_F14: DWORD = 0x7D;
pub const KEY_F15: DWORD = 0x7E;
pub const KEY_F16: DWORD = 0x7F;
pub const KEY_F17: DWORD = 0x80;
pub const KEY_F18: DWORD = 0x81;
pub const KEY_F19: DWORD = 0x82;
pub const KEY_F20: DWORD = 0x83;
pub const KEY_F21: DWORD = 0x84;
pub const KEY_F22: DWORD = 0x85;
pub const KEY_F23: DWORD = 0x86;
pub const KEY_F24: DWORD = 0x87;
pub const KEY_NAVIGATION_VIEW: DWORD = 0x88;
pub const KEY_NAVIGATION_MENU: DWORD = 0x89;
pub const KEY_NAVIGATION_UP: DWORD = 0x8A;
pub const KEY_NAVIGATION_DOWN: DWORD = 0x8B;
pub const KEY_NAVIGATION_LEFT: DWORD = 0x8C;
pub const KEY_NAVIGATION_RIGHT: DWORD = 0x8D;
pub const KEY_NAVIGATION_ACCEPT: DWORD = 0x8E;
pub const KEY_NAVIGATION_CANCEL: DWORD = 0x8F;
pub const KEY_SCROLL: DWORD = 0x91;
pub const KEY_OEM_NEC_EQUAL: DWORD = 0x92;
pub const KEY_OEM_FJ_JISHO: DWORD = 0x92;
pub const KEY_OEM_FJ_MASSHOU: DWORD = 0x93;
pub const KEY_OEM_FJ_TOUROKU: DWORD = 0x94;
pub const KEY_OEM_FJ_LOYA: DWORD = 0x95;
pub const KEY_OEM_FJ_ROYA: DWORD = 0x96;
pub const KEY_LEFTALT: DWORD = 0xA4;
pub const KEY_RIGHTALT: DWORD = 0xA5;
pub const KEY_BROWSER_BACK: DWORD = 0xA6;
pub const KEY_BROWSER_FORWARD: DWORD = 0xA7;
pub const KEY_BROWSER_REFRESH: DWORD = 0xA8;
pub const KEY_BROWSER_STOP: DWORD = 0xA9;
pub const KEY_BROWSER_SEARCH: DWORD = 0xAA;
pub const KEY_BROWSER_FAVORITES: DWORD = 0xAB;
pub const KEY_BROWSER_HOME: DWORD = 0xAC;
pub const KEY_VOLUME_MUTE: DWORD = 0xAD;
pub const KEY_VOLUME_DOWN: DWORD = 0xAE;
pub const KEY_VOLUME_UP: DWORD = 0xAF;
pub const KEY_MEDIA_NEXT_TRACK: DWORD = 0xB0;
pub const KEY_MEDIA_PREV_TRACK: DWORD = 0xB1;
pub const KEY_MEDIA_STOP: DWORD = 0xB2;
pub const KEY_MEDIA_PLAY_PAUSE: DWORD = 0xB3;
pub const KEY_LAUNCH_MAIL: DWORD = 0xB4;
pub const KEY_LAUNCH_MEDIA_SELECT: DWORD = 0xB5;
pub const KEY_LAUNCH_APP1: DWORD = 0xB6;
pub const KEY_LAUNCH_APP2: DWORD = 0xB7;
pub const KEY_SEMICOLON: DWORD = 0xBA;
pub const KEY_EQUAL: DWORD = 0xBB;
pub const KEY_COMMA: DWORD = 0xBC;
pub const KEY_MINUS: DWORD = 0xBD;
pub const KEY_DOT: DWORD = 0xBE;
pub const KEY_SLASH: DWORD = 0xBF;
pub const KEY_GRAVE: DWORD = 0xC0;
pub const KEY_GAMEPAD_A: DWORD = 0xC3;
pub const KEY_GAMEPAD_B: DWORD = 0xC4;
pub const KEY_GAMEPAD_X: DWORD = 0xC5;
pub const KEY_GAMEPAD_Y: DWORD = 0xC6;
pub const KEY_GAMEPAD_RIGHT_SHOULDER: DWORD = 0xC7;
pub const KEY_GAMEPAD_LEFT_SHOULDER: DWORD = 0xC8;
pub const KEY_GAMEPAD_LEFT_TRIGGER: DWORD = 0xC9;
pub const KEY_GAMEPAD_RIGHT_TRIGGER: DWORD = 0xCA;
pub const KEY_GAMEPAD_DPAD_UP: DWORD = 0xCB;
pub const KEY_GAMEPAD_DPAD_DOWN: DWORD = 0xCC;
pub const KEY_GAMEPAD_DPAD_LEFT: DWORD = 0xCD;
pub const KEY_GAMEPAD_DPAD_RIGHT: DWORD = 0xCE;
pub const KEY_GAMEPAD_MENU: DWORD = 0xCF;
pub const KEY_GAMEPAD_VIEW: DWORD = 0xD0;
pub const KEY_GAMEPAD_LEFT_THUMBSTICK_BUTTON: DWORD = 0xD1;
pub const KEY_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: DWORD = 0xD2;
pub const KEY_GAMEPAD_LEFT_THUMBSTICK_UP: DWORD = 0xD3;
pub const KEY_GAMEPAD_LEFT_THUMBSTICK_DOWN: DWORD = 0xD4;
pub const KEY_GAMEPAD_LEFT_THUMBSTICK_RIGHT: DWORD = 0xD5;
pub const KEY_GAMEPAD_LEFT_THUMBSTICK_LEFT: DWORD = 0xD6;
pub const KEY_GAMEPAD_RIGHT_THUMBSTICK_UP: DWORD = 0xD7;
pub const KEY_GAMEPAD_RIGHT_THUMBSTICK_DOWN: DWORD = 0xD8;
pub const KEY_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: DWORD = 0xD9;
pub const KEY_GAMEPAD_RIGHT_THUMBSTICK_LEFT: DWORD = 0xDA;
pub const KEY_LEFTBRACE: DWORD = 0xDB;
pub const KEY_BACKSLASH: DWORD = 0xDC;
pub const KEY_RIGHTBRACE: DWORD = 0xDD;
pub const KEY_APOSTROPHE: DWORD = 0xDE;
pub const KEY_OEM_8: DWORD = 0xDF;
pub const KEY_OEM_AX: DWORD = 0xE1;
pub const KEY_OEM_102: DWORD = 0xE2;
pub const KEY_ICO_HELP: DWORD = 0xE3;
pub const KEY_ICO_00: DWORD = 0xE4;
pub const KEY_PROCESSKEY: DWORD = 0xE5;
pub const KEY_ICO_CLEAR: DWORD = 0xE6;
pub const KEY_PACKET: DWORD = 0xE7;
pub const KEY_OEM_RESET: DWORD = 0xE9;
pub const KEY_OEM_JUMP: DWORD = 0xEA;
pub const KEY_OEM_PA1: DWORD = 0xEB;
pub const KEY_OEM_PA2: DWORD = 0xEC;
pub const KEY_OEM_PA3: DWORD = 0xED;
pub const KEY_OEM_WSCTRL: DWORD = 0xEE;
pub const KEY_OEM_CUSEL: DWORD = 0xEF;
pub const KEY_OEM_ATTN: DWORD = 0xF0;
pub const KEY_OEM_FINISH: DWORD = 0xF1;
pub const KEY_OEM_COPY: DWORD = 0xF2;
pub const KEY_OEM_AUTO: DWORD = 0xF3;
pub const KEY_OEM_ENLW: DWORD = 0xF4;
pub const KEY_OEM_BACKTAB: DWORD = 0xF5;
pub const KEY_ATTN: DWORD = 0xF6;
pub const KEY_CRSEL: DWORD = 0xF7;
pub const KEY_EXSEL: DWORD = 0xF8;
pub const KEY_EREOF: DWORD = 0xF9;
pub const KEY_PLAY: DWORD = 0xFA;
pub const KEY_ZOOM: DWORD = 0xFB;
pub const KEY_NONAME: DWORD = 0xFC;
pub const KEY_PA1: DWORD = 0xFD;
pub const KEY_OEM_CLEAR: DWORD = 0xFE;

pub fn key_map() -> HashMap<&'static str, USizeableDWORD> {
    [
        // grep 'Key => 0x' ../rusty-keys-win/src/windows/inputs.rs | tr '[a-z]' '[A-Z]' | sed -r -e 's/KEY => 0X/: DWORD = 0x/' -e 's/^[ ]+/pub const KEY_/' | tr ',' ';'
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
        /*
        ("ZENKAKUHANKAKU", KEY_ZENKAKUHANKAKU),
        ("102ND", KEY_102ND),
        */
        ("F11", KEY_F11),
        ("F12", KEY_F12),
        /*
        ("RO", KEY_RO),
        ("KATAKANA", KEY_KATAKANA),
        ("HIRAGANA", KEY_HIRAGANA),
        ("HENKAN", KEY_HENKAN),
        ("KATAKANAHIRAGANA", KEY_KATAKANAHIRAGANA),
        ("MUHENKAN", KEY_MUHENKAN),
        ("KPJPCOMMA", KEY_KPJPCOMMA),
        */
        ("KPENTER", KEY_KPENTER),
        ("RIGHTCTRL", KEY_RIGHTCTRL),
        ("KPSLASH", KEY_KPSLASH),
        ("SYSRQ", KEY_SYSRQ),
        ("RIGHTALT", KEY_RIGHTALT),
        /*
        ("LINEFEED", KEY_LINEFEED),
        */
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
        /*
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
        */
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
        /*
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
        */
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
    ]
    .iter()
    .cloned()
    .map(|(m, v)| (m, USizeableDWORD(v)))
    .collect()
}

// https://www.reddit.com/r/rust/comments/9xth8g/why_no_from_conversion_from_u32_to_usize/
#[derive(Copy, Clone, Hash, Eq, PartialEq, Default)]
pub struct USizeableDWORD(pub DWORD);

impl From<USizeableDWORD> for usize {
    fn from(u: USizeableDWORD) -> Self {
        u.0 as usize
    }
}

impl TryFrom<usize> for USizeableDWORD {
    type Error = ();

    fn try_from(value: usize) -> ::std::result::Result<Self, Self::Error> {
        Ok(USizeableDWORD(value as DWORD))
    }
}
