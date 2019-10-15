
use std::collections::HashMap;
use core_graphics::event::CGKeyCode;

// https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes

pub const KEY_BACKSPACE: CGKeyCode = 0x08;
pub const KEY_TAB: CGKeyCode = 0x09;
pub const KEY_ENTER: CGKeyCode = 0x0D;
pub const KEY_KPENTER: CGKeyCode = 0x0D; // on windows, same as KEY_ENTER..
pub const KEY_ESC: CGKeyCode = 0x1B;
pub const KEY_SPACE: CGKeyCode = 0x20;
pub const KEY_HOME: CGKeyCode = 0x24;
pub const KEY_LEFT: CGKeyCode = 0x25;
pub const KEY_UP: CGKeyCode = 0x26;
pub const KEY_RIGHT: CGKeyCode = 0x27;
pub const KEY_DOWN: CGKeyCode = 0x28;
pub const KEY_INSERT: CGKeyCode = 0x2D;
pub const KEY_DELETE: CGKeyCode = 0x2E;
pub const KEY_10: CGKeyCode = 0x30; // named 10 on linux, 0 on windows..
pub const KEY_0: CGKeyCode = 0x30;
pub const KEY_1: CGKeyCode = 0x31;
pub const KEY_2: CGKeyCode = 0x32;
pub const KEY_3: CGKeyCode = 0x33;
pub const KEY_4: CGKeyCode = 0x34;
pub const KEY_5: CGKeyCode = 0x35;
pub const KEY_6: CGKeyCode = 0x36;
pub const KEY_7: CGKeyCode = 0x37;
pub const KEY_8: CGKeyCode = 0x38;
pub const KEY_9: CGKeyCode = 0x39;
pub const KEY_A: CGKeyCode = 0x41;
pub const KEY_B: CGKeyCode = 0x42;
pub const KEY_C: CGKeyCode = 0x43;
pub const KEY_D: CGKeyCode = 0x44;
pub const KEY_E: CGKeyCode = 0x45;
pub const KEY_F: CGKeyCode = 0x46;
pub const KEY_G: CGKeyCode = 0x47;
pub const KEY_H: CGKeyCode = 0x48;
pub const KEY_I: CGKeyCode = 0x49;
pub const KEY_J: CGKeyCode = 0x4A;
pub const KEY_K: CGKeyCode = 0x4B;
pub const KEY_L: CGKeyCode = 0x4C;
pub const KEY_M: CGKeyCode = 0x4D;
pub const KEY_N: CGKeyCode = 0x4E;
pub const KEY_O: CGKeyCode = 0x4F;
pub const KEY_P: CGKeyCode = 0x50;
pub const KEY_Q: CGKeyCode = 0x51;
pub const KEY_R: CGKeyCode = 0x52;
pub const KEY_S: CGKeyCode = 0x53;
pub const KEY_T: CGKeyCode = 0x54;
pub const KEY_U: CGKeyCode = 0x55;
pub const KEY_V: CGKeyCode = 0x56;
pub const KEY_W: CGKeyCode = 0x57;
pub const KEY_X: CGKeyCode = 0x58;
pub const KEY_Y: CGKeyCode = 0x59;
pub const KEY_Z: CGKeyCode = 0x5A;
pub const KEY_KP0: CGKeyCode = 0x60;
pub const KEY_KP1: CGKeyCode = 0x61;
pub const KEY_KP2: CGKeyCode = 0x62;
pub const KEY_KP3: CGKeyCode = 0x63;
pub const KEY_KP4: CGKeyCode = 0x64;
pub const KEY_KP5: CGKeyCode = 0x65;
pub const KEY_KP6: CGKeyCode = 0x66;
pub const KEY_KP7: CGKeyCode = 0x67;
pub const KEY_KP8: CGKeyCode = 0x68;
pub const KEY_KP9: CGKeyCode = 0x69;
pub const KEY_F1: CGKeyCode = 0x70;
pub const KEY_F2: CGKeyCode = 0x71;
pub const KEY_F3: CGKeyCode = 0x72;
pub const KEY_F4: CGKeyCode = 0x73;
pub const KEY_F5: CGKeyCode = 0x74;
pub const KEY_F6: CGKeyCode = 0x75;
pub const KEY_F7: CGKeyCode = 0x76;
pub const KEY_F8: CGKeyCode = 0x77;
pub const KEY_F9: CGKeyCode = 0x78;
pub const KEY_F10: CGKeyCode = 0x79;
pub const KEY_F11: CGKeyCode = 0x7A;
pub const KEY_F12: CGKeyCode = 0x7B;
pub const KEY_NUMLOCK: CGKeyCode = 0x90;
pub const KEY_SCROLLLOCK: CGKeyCode = 0x91;
pub const KEY_CAPSLOCK: CGKeyCode = 0x14;
pub const KEY_LEFTSHIFT: CGKeyCode = 0xA0;
pub const KEY_RIGHTSHIFT: CGKeyCode = 0xA1;
pub const KEY_LEFTCTRL: CGKeyCode = 0xA2;
pub const KEY_RIGHTCTRL: CGKeyCode = 0xA3;

pub const KEY_LBUTTON: CGKeyCode = 0x01;
pub const KEY_RBUTTON: CGKeyCode = 0x02;
pub const KEY_CANCEL: CGKeyCode = 0x03;
pub const KEY_MBUTTON: CGKeyCode = 0x04;
pub const KEY_XBUTTON1: CGKeyCode = 0x05;
pub const KEY_XBUTTON2: CGKeyCode = 0x06;
pub const KEY_BACK: CGKeyCode = 0x08;
pub const KEY_CLEAR: CGKeyCode = 0x0C;
pub const KEY_RETURN: CGKeyCode = 0x0D;
pub const KEY_SHIFT: CGKeyCode = 0x10;
pub const KEY_CONTROL: CGKeyCode = 0x11;
pub const KEY_MENU: CGKeyCode = 0x12;
pub const KEY_PAUSE: CGKeyCode = 0x13;
pub const KEY_CAPITAL: CGKeyCode = 0x14;
pub const KEY_KANA: CGKeyCode = 0x15;
pub const KEY_HANGEUL: CGKeyCode = 0x15;
pub const KEY_HANGUL: CGKeyCode = 0x15;
pub const KEY_JUNJA: CGKeyCode = 0x17;
pub const KEY_FINAL: CGKeyCode = 0x18;
pub const KEY_HANJA: CGKeyCode = 0x19;
pub const KEY_KANJI: CGKeyCode = 0x19;
pub const KEY_ESCAPE: CGKeyCode = 0x1B;
pub const KEY_CONVERT: CGKeyCode = 0x1C;
pub const KEY_NONCONVERT: CGKeyCode = 0x1D;
pub const KEY_ACCEPT: CGKeyCode = 0x1E;
pub const KEY_MODECHANGE: CGKeyCode = 0x1F;
pub const KEY_PAGEUP: CGKeyCode = 0x21;
pub const KEY_PAGEDOWN: CGKeyCode = 0x22;
pub const KEY_END: CGKeyCode = 0x23;
pub const KEY_SELECT: CGKeyCode = 0x29;
pub const KEY_PRINT: CGKeyCode = 0x2A;
pub const KEY_EXECUTE: CGKeyCode = 0x2B;
pub const KEY_SYSRQ: CGKeyCode = 0x2C;
pub const KEY_HELP: CGKeyCode = 0x2F;
pub const KEY_LEFTMETA: CGKeyCode = 0x5B;
pub const KEY_RIGHTMETA: CGKeyCode = 0x5C;
pub const KEY_COMPOSE: CGKeyCode = 0x5D;
pub const KEY_SLEEP: CGKeyCode = 0x5F;
pub const KEY_KPASTERISK: CGKeyCode = 0x6A;
pub const KEY_KPPLUS: CGKeyCode = 0x6B;
pub const KEY_SEPARATOR: CGKeyCode = 0x6C;
pub const KEY_KPMINUS: CGKeyCode = 0x6D;
pub const KEY_KPDOT: CGKeyCode = 0x6E;
pub const KEY_KPSLASH: CGKeyCode = 0x6F;
pub const KEY_F13: CGKeyCode = 0x7C;
pub const KEY_F14: CGKeyCode = 0x7D;
pub const KEY_F15: CGKeyCode = 0x7E;
pub const KEY_F16: CGKeyCode = 0x7F;
pub const KEY_F17: CGKeyCode = 0x80;
pub const KEY_F18: CGKeyCode = 0x81;
pub const KEY_F19: CGKeyCode = 0x82;
pub const KEY_F20: CGKeyCode = 0x83;
pub const KEY_F21: CGKeyCode = 0x84;
pub const KEY_F22: CGKeyCode = 0x85;
pub const KEY_F23: CGKeyCode = 0x86;
pub const KEY_F24: CGKeyCode = 0x87;
pub const KEY_NAVIGATION_VIEW: CGKeyCode = 0x88;
pub const KEY_NAVIGATION_MENU: CGKeyCode = 0x89;
pub const KEY_NAVIGATION_UP: CGKeyCode = 0x8A;
pub const KEY_NAVIGATION_DOWN: CGKeyCode = 0x8B;
pub const KEY_NAVIGATION_LEFT: CGKeyCode = 0x8C;
pub const KEY_NAVIGATION_RIGHT: CGKeyCode = 0x8D;
pub const KEY_NAVIGATION_ACCEPT: CGKeyCode = 0x8E;
pub const KEY_NAVIGATION_CANCEL: CGKeyCode = 0x8F;
pub const KEY_SCROLL: CGKeyCode = 0x91;
pub const KEY_OEM_NEC_EQUAL: CGKeyCode = 0x92;
pub const KEY_OEM_FJ_JISHO: CGKeyCode = 0x92;
pub const KEY_OEM_FJ_MASSHOU: CGKeyCode = 0x93;
pub const KEY_OEM_FJ_TOUROKU: CGKeyCode = 0x94;
pub const KEY_OEM_FJ_LOYA: CGKeyCode = 0x95;
pub const KEY_OEM_FJ_ROYA: CGKeyCode = 0x96;
pub const KEY_LEFTALT: CGKeyCode = 0xA4;
pub const KEY_RIGHTALT: CGKeyCode = 0xA5;
pub const KEY_BROWSER_BACK: CGKeyCode = 0xA6;
pub const KEY_BROWSER_FORWARD: CGKeyCode = 0xA7;
pub const KEY_BROWSER_REFRESH: CGKeyCode = 0xA8;
pub const KEY_BROWSER_STOP: CGKeyCode = 0xA9;
pub const KEY_BROWSER_SEARCH: CGKeyCode = 0xAA;
pub const KEY_BROWSER_FAVORITES: CGKeyCode = 0xAB;
pub const KEY_BROWSER_HOME: CGKeyCode = 0xAC;
pub const KEY_VOLUME_MUTE: CGKeyCode = 0xAD;
pub const KEY_VOLUME_DOWN: CGKeyCode = 0xAE;
pub const KEY_VOLUME_UP: CGKeyCode = 0xAF;
pub const KEY_MEDIA_NEXT_TRACK: CGKeyCode = 0xB0;
pub const KEY_MEDIA_PREV_TRACK: CGKeyCode = 0xB1;
pub const KEY_MEDIA_STOP: CGKeyCode = 0xB2;
pub const KEY_MEDIA_PLAY_PAUSE: CGKeyCode = 0xB3;
pub const KEY_LAUNCH_MAIL: CGKeyCode = 0xB4;
pub const KEY_LAUNCH_MEDIA_SELECT: CGKeyCode = 0xB5;
pub const KEY_LAUNCH_APP1: CGKeyCode = 0xB6;
pub const KEY_LAUNCH_APP2: CGKeyCode = 0xB7;
pub const KEY_SEMICOLON: CGKeyCode = 0xBA;
pub const KEY_EQUAL: CGKeyCode = 0xBB;
pub const KEY_COMMA: CGKeyCode = 0xBC;
pub const KEY_MINUS: CGKeyCode = 0xBD;
pub const KEY_DOT: CGKeyCode = 0xBE;
pub const KEY_SLASH: CGKeyCode = 0xBF;
pub const KEY_GRAVE: CGKeyCode = 0xC0;
pub const KEY_GAMEPAD_A: CGKeyCode = 0xC3;
pub const KEY_GAMEPAD_B: CGKeyCode = 0xC4;
pub const KEY_GAMEPAD_X: CGKeyCode = 0xC5;
pub const KEY_GAMEPAD_Y: CGKeyCode = 0xC6;
pub const KEY_GAMEPAD_RIGHT_SHOULDER: CGKeyCode = 0xC7;
pub const KEY_GAMEPAD_LEFT_SHOULDER: CGKeyCode = 0xC8;
pub const KEY_GAMEPAD_LEFT_TRIGGER: CGKeyCode = 0xC9;
pub const KEY_GAMEPAD_RIGHT_TRIGGER: CGKeyCode = 0xCA;
pub const KEY_GAMEPAD_DPAD_UP: CGKeyCode = 0xCB;
pub const KEY_GAMEPAD_DPAD_DOWN: CGKeyCode = 0xCC;
pub const KEY_GAMEPAD_DPAD_LEFT: CGKeyCode = 0xCD;
pub const KEY_GAMEPAD_DPAD_RIGHT: CGKeyCode = 0xCE;
pub const KEY_GAMEPAD_MENU: CGKeyCode = 0xCF;
pub const KEY_GAMEPAD_VIEW: CGKeyCode = 0xD0;
pub const KEY_GAMEPAD_LEFT_THUMBSTICK_BUTTON: CGKeyCode = 0xD1;
pub const KEY_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: CGKeyCode = 0xD2;
pub const KEY_GAMEPAD_LEFT_THUMBSTICK_UP: CGKeyCode = 0xD3;
pub const KEY_GAMEPAD_LEFT_THUMBSTICK_DOWN: CGKeyCode = 0xD4;
pub const KEY_GAMEPAD_LEFT_THUMBSTICK_RIGHT: CGKeyCode = 0xD5;
pub const KEY_GAMEPAD_LEFT_THUMBSTICK_LEFT: CGKeyCode = 0xD6;
pub const KEY_GAMEPAD_RIGHT_THUMBSTICK_UP: CGKeyCode = 0xD7;
pub const KEY_GAMEPAD_RIGHT_THUMBSTICK_DOWN: CGKeyCode = 0xD8;
pub const KEY_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: CGKeyCode = 0xD9;
pub const KEY_GAMEPAD_RIGHT_THUMBSTICK_LEFT: CGKeyCode = 0xDA;
pub const KEY_LEFTBRACE: CGKeyCode = 0xDB;
pub const KEY_BACKSLASH: CGKeyCode = 0xDC;
pub const KEY_RIGHTBRACE: CGKeyCode = 0xDD;
pub const KEY_APOSTROPHE: CGKeyCode = 0xDE;
pub const KEY_OEM_8: CGKeyCode = 0xDF;
pub const KEY_OEM_AX: CGKeyCode = 0xE1;
pub const KEY_OEM_102: CGKeyCode = 0xE2;
pub const KEY_ICO_HELP: CGKeyCode = 0xE3;
pub const KEY_ICO_00: CGKeyCode = 0xE4;
pub const KEY_PROCESSKEY: CGKeyCode = 0xE5;
pub const KEY_ICO_CLEAR: CGKeyCode = 0xE6;
pub const KEY_PACKET: CGKeyCode = 0xE7;
pub const KEY_OEM_RESET: CGKeyCode = 0xE9;
pub const KEY_OEM_JUMP: CGKeyCode = 0xEA;
pub const KEY_OEM_PA1: CGKeyCode = 0xEB;
pub const KEY_OEM_PA2: CGKeyCode = 0xEC;
pub const KEY_OEM_PA3: CGKeyCode = 0xED;
pub const KEY_OEM_WSCTRL: CGKeyCode = 0xEE;
pub const KEY_OEM_CUSEL: CGKeyCode = 0xEF;
pub const KEY_OEM_ATTN: CGKeyCode = 0xF0;
pub const KEY_OEM_FINISH: CGKeyCode = 0xF1;
pub const KEY_OEM_COPY: CGKeyCode = 0xF2;
pub const KEY_OEM_AUTO: CGKeyCode = 0xF3;
pub const KEY_OEM_ENLW: CGKeyCode = 0xF4;
pub const KEY_OEM_BACKTAB: CGKeyCode = 0xF5;
pub const KEY_ATTN: CGKeyCode = 0xF6;
pub const KEY_CRSEL: CGKeyCode = 0xF7;
pub const KEY_EXSEL: CGKeyCode = 0xF8;
pub const KEY_EREOF: CGKeyCode = 0xF9;
pub const KEY_PLAY: CGKeyCode = 0xFA;
pub const KEY_ZOOM: CGKeyCode = 0xFB;
pub const KEY_NONAME: CGKeyCode = 0xFC;
pub const KEY_PA1: CGKeyCode = 0xFD;
pub const KEY_OEM_CLEAR: CGKeyCode = 0xFE;

pub fn key_map() -> HashMap<&'static str, CGKeyCode> {
    [
        // grep 'Key => 0x' ../rusty-keys-win/src/windows/inputs.rs | tr '[a-z]' '[A-Z]' | sed -r -e 's/KEY => 0X/: CGKeyCode = 0x/' -e 's/^[ ]+/pub const KEY_/' | tr ',' ';'
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
    ].iter().cloned().map(|(m, v)| (m, v)).collect()
}
