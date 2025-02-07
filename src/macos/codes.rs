use core_graphics::event::CGKeyCode;
use std::collections::HashMap;

pub const KEY_RESERVED: CGKeyCode = 0x31;
pub const KEY_ESC: CGKeyCode = 0x00;
pub const KEY_1: CGKeyCode = 0x0012;
pub const KEY_2: CGKeyCode = 0x0013;
pub const KEY_3: CGKeyCode = 0x0014;
pub const KEY_4: CGKeyCode = 0x0015;
pub const KEY_5: CGKeyCode = 0x0017;
pub const KEY_6: CGKeyCode = 0x0016;
pub const KEY_7: CGKeyCode = 0x001A;
pub const KEY_8: CGKeyCode = 0x001C;
pub const KEY_9: CGKeyCode = 0x0019;
pub const KEY_10: CGKeyCode = 0x001D;
pub const KEY_MINUS: CGKeyCode = 0x001B;
pub const KEY_EQUAL: CGKeyCode = 0x0018;
pub const KEY_BACKSPACE: CGKeyCode = 0x0033;
pub const KEY_TAB: CGKeyCode = 0x0030;
pub const KEY_Q: CGKeyCode = 0x000C;
pub const KEY_W: CGKeyCode = 0x000D;
pub const KEY_E: CGKeyCode = 0x000E;
pub const KEY_R: CGKeyCode = 0x000F;
pub const KEY_T: CGKeyCode = 0x0011;
pub const KEY_Y: CGKeyCode = 0x0010;
pub const KEY_U: CGKeyCode = 0x0020;
pub const KEY_I: CGKeyCode = 0x0022;
pub const KEY_O: CGKeyCode = 0x001F;
pub const KEY_P: CGKeyCode = 0x0023;
pub const KEY_LEFTBRACE: CGKeyCode = 0x0021;
pub const KEY_RIGHTBRACE: CGKeyCode = 0x001E;
pub const KEY_ENTER: CGKeyCode = 0x0024;
pub const KEY_LEFTCTRL: CGKeyCode = 0x003B;
pub const KEY_A: CGKeyCode = 0x0000;
pub const KEY_S: CGKeyCode = 0x0001;
pub const KEY_D: CGKeyCode = 0x0002;
pub const KEY_F: CGKeyCode = 0x0003;
pub const KEY_G: CGKeyCode = 0x0005;
pub const KEY_H: CGKeyCode = 0x0004;
pub const KEY_J: CGKeyCode = 0x0026;
pub const KEY_K: CGKeyCode = 0x0028;
pub const KEY_L: CGKeyCode = 0x0025;
pub const KEY_SEMICOLON: CGKeyCode = 0x0029;
pub const KEY_APOSTROPHE: CGKeyCode = 0x0027;
pub const KEY_GRAVE: CGKeyCode = 0x0032;
pub const KEY_LEFTSHIFT: CGKeyCode = 0x0038;
pub const KEY_BACKSLASH: CGKeyCode = 0x002A;
pub const KEY_Z: CGKeyCode = 0x0006;
pub const KEY_X: CGKeyCode = 0x0007;
pub const KEY_C: CGKeyCode = 0x0008;
pub const KEY_V: CGKeyCode = 0x0009;
pub const KEY_B: CGKeyCode = 0x000B;
pub const KEY_N: CGKeyCode = 0x002D;
pub const KEY_M: CGKeyCode = 0x002E;
pub const KEY_COMMA: CGKeyCode = 0x002B;
pub const KEY_DOT: CGKeyCode = 0x002F;
pub const KEY_SLASH: CGKeyCode = 0x002C;
pub const KEY_RIGHTSHIFT: CGKeyCode = 0x003C;
pub const KEY_KPASTERISK: CGKeyCode = 0x0043;
pub const KEY_LEFTALT: CGKeyCode = 0x003A;
pub const KEY_SPACE: CGKeyCode = 0x0031;
pub const KEY_CAPSLOCK: CGKeyCode = 0x0039;
pub const KEY_F1: CGKeyCode = 0x007A;
pub const KEY_F2: CGKeyCode = 0x0078;
pub const KEY_F3: CGKeyCode = 0x0063;
pub const KEY_F4: CGKeyCode = 0x0076;
pub const KEY_F5: CGKeyCode = 0x0060;
pub const KEY_F6: CGKeyCode = 0x0061;
pub const KEY_F7: CGKeyCode = 0x0062;
pub const KEY_F8: CGKeyCode = 0x0064;
pub const KEY_F9: CGKeyCode = 0x0065;
pub const KEY_F10: CGKeyCode = 0x006D;
pub const KEY_NUMLOCK: CGKeyCode = 0x0047;
pub const KEY_SCROLLLOCK: CGKeyCode = 0x006B;
pub const KEY_KP7: CGKeyCode = 0x0059;
pub const KEY_KP8: CGKeyCode = 0x005B;
pub const KEY_KP9: CGKeyCode = 0x005C;
pub const KEY_KPMINUS: CGKeyCode = 0x004E;
pub const KEY_KP4: CGKeyCode = 0x0056;
pub const KEY_KP5: CGKeyCode = 0x0057;
pub const KEY_KP6: CGKeyCode = 0x0058;
pub const KEY_KPPLUS: CGKeyCode = 0x0045;
pub const KEY_KP1: CGKeyCode = 0x0053;
pub const KEY_KP2: CGKeyCode = 0x0054;
pub const KEY_KP3: CGKeyCode = 0x0055;
pub const KEY_KP0: CGKeyCode = 0x0052; // https://code-with-me.jetbrains.com/UCsz0dzSd1QAbmOi8g0V3w#p=IC&fp=6298EDAF97FA62E9897E2556D1A6631FB66974568C7252E696472EE85078E8A0
pub const KEY_KPDOT: CGKeyCode = 0x0041;
// pub const KEY_ZENKAKUHANKAKU: CGKeyCode = NOT_FOUND;
pub const KEY_102ND: CGKeyCode = 0x000A;
pub const KEY_F11: CGKeyCode = 0x0067;
pub const KEY_F12: CGKeyCode = 0x006F;
// pub const KEY_RO: CGKeyCode = NOT_FOUND;
// pub const KEY_KATAKANA: CGKeyCode = NOT_FOUND;
// pub const KEY_HIRAGANA: CGKeyCode = NOT_FOUND;
// pub const KEY_HENKAN: CGKeyCode = NOT_FOUND;
// pub const KEY_KATAKANAHIRAGANA: CGKeyCode = NOT_FOUND;
// pub const KEY_MUHENKAN: CGKeyCode = NOT_FOUND;
// pub const KEY_KPJPCOMMA: CGKeyCode = NOT_FOUND; // https://code-with-me.jetbrains.com/Mf59EFUeJZQ2mpGCCCjNWw#p=IC&fp=6298EDAF97FA62E9897E2556D1A6631FB66974568C7252E696472EE85078E8A0
pub const KEY_KPENTER: CGKeyCode = 0x004C;
pub const KEY_RIGHTCTRL: CGKeyCode = 0x003E;
pub const KEY_KPSLASH: CGKeyCode = 0x004B;
pub const KEY_SYSRQ: CGKeyCode = 0x0069;
pub const KEY_RIGHTALT: CGKeyCode = 0x003D;
pub const KEY_LINEFEED: CGKeyCode = 0x0071;
pub const KEY_HOME: CGKeyCode = 0x0073;
pub const KEY_UP: CGKeyCode = 0x007E;
pub const KEY_PAGEUP: CGKeyCode = 0x0074;
pub const KEY_LEFT: CGKeyCode = 0x007B;
pub const KEY_RIGHT: CGKeyCode = 0x007C;
pub const KEY_END: CGKeyCode = 0x0077;
pub const KEY_DOWN: CGKeyCode = 0x007D;
pub const KEY_PAGEDOWN: CGKeyCode = 0x0079;
pub const KEY_INSERT: CGKeyCode = 0x0072;
pub const KEY_DELETE: CGKeyCode = 0x0075;
// pub const KEY_MACRO: CGKeyCode = NOT_FOUND;
// pub const KEY_MUTE: CGKeyCode = NOT_FOUND;
// pub const KEY_VOLUMEDOWN: CGKeyCode = NOT_FOUND;
// pub const KEY_VOLUMEUP: CGKeyCode = NOT_FOUND;
// pub const KEY_POWER: CGKeyCode = NOT_FOUND;
pub const KEY_KPEQUAL: CGKeyCode = 0x0069;
// pub const KEY_KPPLUSMINUS: CGKeyCode = NOT_FOUND;
pub const KEY_PAUSE: CGKeyCode = KEY_LINEFEED;
pub const KEY_SCALE: CGKeyCode = 0x0047;
pub const KEY_KPCOMMA: CGKeyCode = 0x0036;
// pub const KEY_HANGEUL: CGKeyCode = NOT_FOUND;
// pub const KEY_HANGUEL: CGKeyCode = NOT_FOUND;
// pub const KEY_HANGEUL: CGKeyCode = NOT_FOUND;
// pub const KEY_HANJA: CGKeyCode = NOT_FOUND;
//pub const KEY_YEN: CGKeyCode = NOT_FOUND;
pub const KEY_LEFTMETA: CGKeyCode = 0x0037;
pub const KEY_RIGHTMETA: CGKeyCode = 0x0036;
pub const KEY_COMPOSE: CGKeyCode = 0x006E;
// pub const KEY_STOP: CGKeyCode = NOT_FOUND;
// pub const KEY_AGAIN: CGKeyCode = NOT_FOUND;
// pub const KEY_PROPS: CGKeyCode = NOT_FOUND;
// pub const KEY_UNDO: CGKeyCode = NOT_FOUND;
// pub const KEY_FRONT: CGKeyCode = NOT_FOUND;
// pub const KEY_COPY: CGKeyCode = NOT_FOUND;
// pub const KEY_OPEN: CGKeyCode = NOT_FOUND;
// pub const KEY_PASTE: CGKeyCode = NOT_FOUND;
// pub const KEY_FIND: CGKeyCode = NOT_FOUND;
// pub const KEY_CUT: CGKeyCode = NOT_FOUND;
// pub const KEY_HELP: CGKeyCode = NOT_FOUND;
// pub const KEY_MENU: CGKeyCode = NOT_FOUND;
// pub const KEY_CALC: CGKeyCode = NOT_FOUND;
// pub const KEY_SETUP: CGKeyCode = NOT_FOUND;
// pub const KEY_SLEEP: CGKeyCode = NOT_FOUND;
// pub const KEY_WAKEUP: CGKeyCode = NOT_FOUND;
// pub const KEY_FILE: CGKeyCode = NOT_FOUND;
// pub const KEY_SENDFILE: CGKeyCode = NOT_FOUND;
// pub const KEY_DELETEFILE: CGKeyCode = NOT_FOUND;
// pub const KEY_XFER: CGKeyCode = NOT_FOUND;
// pub const KEY_PROG1: CGKeyCode = NOT_FOUND;
// pub const KEY_PROG2: CGKeyCode = NOT_FOUND;
// pub const KEY_WWW: CGKeyCode = NOT_FOUND;
// pub const KEY_MSDOS: CGKeyCode = NOT_FOUND;
// pub const KEY_COFFEE: CGKeyCode = NOT_FOUND;
// pub const KEY_SCREENLOCK: CGKeyCode = NOT_FOUND;
// pub const KEY_COFFEE: CGKeyCode = NOT_FOUND;
// pub const KEY_ROTATE_DISPLAY: CGKeyCode = NOT_FOUND;
// pub const KEY_DIRECTION: CGKeyCode = NOT_FOUND;
// pub const KEY_ROTATE_DISPLAY: CGKeyCode = NOT_FOUND;
// pub const KEY_CYCLEWINDOWS: CGKeyCode = NOT_FOUND;
// pub const KEY_MAIL: CGKeyCode = NOT_FOUND;
// pub const KEY_BOOKMARKS: CGKeyCode = NOT_FOUND;
// pub const KEY_COMPUTER: CGKeyCode = NOT_FOUND;
// pub const KEY_BACK: CGKeyCode = NOT_FOUND;
// pub const KEY_FORWARD: CGKeyCode = NOT_FOUND;
// pub const KEY_CLOSECD: CGKeyCode = NOT_FOUND;
// pub const KEY_EJECTCD: CGKeyCode = NOT_FOUND;
// pub const KEY_EJECTCLOSECD: CGKeyCode = NOT_FOUND;
// pub const KEY_NEXTSONG: CGKeyCode = NOT_FOUND;
// pub const KEY_PLAYPAUSE: CGKeyCode = NOT_FOUND;
// pub const KEY_PREVIOUSSONG: CGKeyCode = NOT_FOUND;
// pub const KEY_STOPCD: CGKeyCode = NOT_FOUND;
// pub const KEY_RECORD: CGKeyCode = NOT_FOUND;
// pub const KEY_REWIND: CGKeyCode = NOT_FOUND;
// pub const KEY_PHONE: CGKeyCode = NOT_FOUND;
// pub const KEY_ISO: CGKeyCode = NOT_FOUND;
// pub const KEY_CONFIG: CGKeyCode = NOT_FOUND;
// pub const KEY_HOMEPAGE: CGKeyCode = NOT_FOUND;
// pub const KEY_REFRESH: CGKeyCode = NOT_FOUND;
// pub const KEY_EXIT: CGKeyCode = NOT_FOUND;
// pub const KEY_MOVE: CGKeyCode = NOT_FOUND;
// pub const KEY_EDIT: CGKeyCode = NOT_FOUND;
// pub const KEY_SCROLLUP: CGKeyCode = NOT_FOUND;
// pub const KEY_SCROLLDOWN: CGKeyCode = NOT_FOUND;
// pub const KEY_KPLEFTPAREN: CGKeyCode = NOT_FOUND;
// pub const KEY_KPRIGHTPAREN: CGKeyCode = NOT_FOUND;
// pub const KEY_NEW: CGKeyCode = NOT_FOUND;
// pub const KEY_REDO: CGKeyCode = NOT_FOUND;
// pub const KEY_F13: CGKeyCode = NOT_FOUND;
// pub const KEY_F14: CGKeyCode = NOT_FOUND;
// pub const KEY_F15: CGKeyCode = NOT_FOUND;
// pub const KEY_F16: CGKeyCode = NOT_FOUND;
// pub const KEY_F17: CGKeyCode = NOT_FOUND;
// pub const KEY_F18: CGKeyCode = NOT_FOUND;
// pub const KEY_F19: CGKeyCode = NOT_FOUND;
// pub const KEY_F20: CGKeyCode = NOT_FOUND;
// pub const KEY_F21: CGKeyCode = NOT_FOUND;
// pub const KEY_F22: CGKeyCode = NOT_FOUND;
// pub const KEY_F23: CGKeyCode = NOT_FOUND;
// pub const KEY_F24: CGKeyCode = NOT_FOUND;
// pub const KEY_PLAYCD: CGKeyCode = NOT_FOUND;
// pub const KEY_PAUSECD: CGKeyCode = NOT_FOUND;
// pub const KEY_PROG3: CGKeyCode = NOT_FOUND;
// pub const KEY_PROG4: CGKeyCode = NOT_FOUND;
// pub const KEY_DASHBOARD: CGKeyCode = NOT_FOUND;
// pub const KEY_SUSPEND: CGKeyCode = NOT_FOUND;
// pub const KEY_CLOSE: CGKeyCode = NOT_FOUND;
// pub const KEY_PLAY: CGKeyCode = NOT_FOUND;
// pub const KEY_FASTFORWARD: CGKeyCode = NOT_FOUND;
// pub const KEY_BASSBOOST: CGKeyCode = NOT_FOUND;
// pub const KEY_PRINT: CGKeyCode = NOT_FOUND;
// pub const KEY_HP: CGKeyCode = NOT_FOUND;
// pub const KEY_CAMERA: CGKeyCode = NOT_FOUND;
// pub const KEY_SOUND: CGKeyCode = NOT_FOUND;
// pub const KEY_QUESTION: CGKeyCode = NOT_FOUND;
// pub const KEY_EMAIL: CGKeyCode = NOT_FOUND;
// pub const KEY_CHAT: CGKeyCode = NOT_FOUND;
// pub const KEY_SEARCH: CGKeyCode = NOT_FOUND;
// pub const KEY_CONNECT: CGKeyCode = NOT_FOUND;
// pub const KEY_FINANCE: CGKeyCode = NOT_FOUND;
// pub const KEY_SPORT: CGKeyCode = NOT_FOUND;
//pub const KEY_SHOP: CGKeyCode = NOT_FOUND;
pub const KEY_ALTERASE: CGKeyCode = 0x0047;
// pub const KEY_CANCEL: CGKeyCode = NOT_FOUND;
// pub const KEY_BRIGHTNESSDOWN: CGKeyCode = NOT_FOUND;
// pub const KEY_BRIGHTNESSUP: CGKeyCode = NOT_FOUND;
// pub const KEY_MEDIA: CGKeyCode = NOT_FOUND;
// pub const KEY_SWITCHVIDEOMODE: CGKeyCode = NOT_FOUND;
// pub const KEY_KBDILLUMTOGGLE: CGKeyCode = NOT_FOUND;
// pub const KEY_KBDILLUMDOWN: CGKeyCode = NOT_FOUND;
// pub const KEY_KBDILLUMUP: CGKeyCode = NOT_FOUND;
// pub const KEY_SEND: CGKeyCode = NOT_FOUND;
// pub const KEY_REPLY: CGKeyCode = NOT_FOUND;
// pub const KEY_FORWARDMAIL: CGKeyCode = NOT_FOUND;
// pub const KEY_SAVE: CGKeyCode = NOT_FOUND;
// pub const KEY_DOCUMENTS: CGKeyCode = NOT_FOUND;
// pub const KEY_BATTERY: CGKeyCode = NOT_FOUND;
// pub const KEY_BLUETOOTH: CGKeyCode = NOT_FOUND;
// pub const KEY_UWB: CGKeyCode = NOT_FOUND;
// pub const KEY_UNKNOWN: CGKeyCode = NOT_FOUND;
// pub const KEY_VIDEO_NEXT: CGKeyCode = NOT_FOUND;
// pub const KEY_VIDEO_PREV: CGKeyCode = NOT_FOUND;
// pub const KEY_BRIGHTNESS_CYCLE: CGKeyCode = NOT_FOUND;
// pub const KEY_BRIGHTNESS_AUTO: CGKeyCode = NOT_FOUND;
// pub const KEY_BRIGHTNESS_ZERO: CGKeyCode = NOT_FOUND;
// pub const KEY_BRIGHTNESS_AUTO: CGKeyCode = NOT_FOUND;
// pub const KEY_DISPLAY_OFF: CGKeyCode = NOT_FOUND;

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
    .map(|(m, v)| (m, v))
    .collect()
}
