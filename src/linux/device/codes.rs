use libc::{c_int};

pub const INPUT_PROP_POINTER:        c_int = 0x00; /* needs a pointer */
pub const INPUT_PROP_DIRECT:         c_int = 0x01; /* direct input devices */
pub const INPUT_PROP_BUTTONPAD:      c_int = 0x02; /* has button: c_int = s under pad */
pub const INPUT_PROP_SEMI_MT:        c_int = 0x03; /* touch rectangle only */
pub const INPUT_PROP_TOPBUTTONPAD:   c_int = 0x04; /* softbuttons at top of pad */
pub const INPUT_PROP_POINTING_STICK: c_int = 0x05; /* is a pointing stick */
pub const INPUT_PROP_ACCELEROMETER:  c_int = 0x06; /* has accelerometer */

pub const INPUT_PROP_MAX: c_int = 0x1f;
pub const INPUT_PROP_CNT: c_int = INPUT_PROP_MAX + 1;

/*
 * Event types
 */

pub const EV_SYN:       c_int = 0x00;
pub const EV_KEY:       c_int = 0x01;
pub const EV_REL:       c_int = 0x02;
pub const EV_ABS:       c_int = 0x03;
pub const EV_MSC:       c_int = 0x04;
pub const EV_SW:        c_int = 0x05;
pub const EV_LED:       c_int = 0x11;
pub const EV_SND:       c_int = 0x12;
pub const EV_REP:       c_int = 0x14;
pub const EV_FF:        c_int = 0x15;
pub const EV_PWR:       c_int = 0x16;
pub const EV_FF_STATUS: c_int = 0x17;
pub const EV_MAX:       c_int = 0x1f;
pub const EV_CNT:       c_int = EV_MAX + 1;

/*
 * Synchronization events.
 */

pub const SYN_REPORT:    c_int = 0;
pub const SYN_CONFIG:    c_int = 1;
pub const SYN_MT_REPORT: c_int = 2;
pub const SYN_DROPPED:   c_int = 3;
pub const SYN_MAX:       c_int = 0xf;
pub const SYN_CNT:       c_int = SYN_MAX + 1;

/*
 * Keys and buttons
 *
 * Most of the keys/buttons are modeled after USB HUT 1.12 (see
 * http://www.usb.org/developers/hidpage).
 * Abbreviations in the comments:
 * AC - Application Control
 * AL - Application Launch Button
 * SC - System Control
 */

pub const KEY_RESERVED:   c_int = 0;
pub const KEY_ESC:        c_int = 1;
pub const KEY_1:          c_int = 2;
pub const KEY_2:          c_int = 3;
pub const KEY_3:          c_int = 4;
pub const KEY_4:          c_int = 5;
pub const KEY_5:          c_int = 6;
pub const KEY_6:          c_int = 7;
pub const KEY_7:          c_int = 8;
pub const KEY_8:          c_int = 9;
pub const KEY_9:          c_int = 10;
pub const KEY_10:         c_int = 11;
pub const KEY_MINUS:      c_int = 12;
pub const KEY_EQUAL:      c_int = 13;
pub const KEY_BACKSPACE:  c_int = 14;
pub const KEY_TAB:        c_int = 15;
pub const KEY_Q:          c_int = 16;
pub const KEY_W:          c_int = 17;
pub const KEY_E:          c_int = 18;
pub const KEY_R:          c_int = 19;
pub const KEY_T:          c_int = 20;
pub const KEY_Y:          c_int = 21;
pub const KEY_U:          c_int = 22;
pub const KEY_I:          c_int = 23;
pub const KEY_O:          c_int = 24;
pub const KEY_P:          c_int = 25;
pub const KEY_LEFTBRACE:  c_int = 26;
pub const KEY_RIGHTBRACE: c_int = 27;
pub const KEY_ENTER:      c_int = 28;
pub const KEY_LEFTCTRL:   c_int = 29;
pub const KEY_A:          c_int = 30;
pub const KEY_S:          c_int = 31;
pub const KEY_D:          c_int = 32;
pub const KEY_F:          c_int = 33;
pub const KEY_G:          c_int = 34;
pub const KEY_H:          c_int = 35;
pub const KEY_J:          c_int = 36;
pub const KEY_K:          c_int = 37;
pub const KEY_L:          c_int = 38;
pub const KEY_SEMICOLON:  c_int = 39;
pub const KEY_APOSTROPHE: c_int = 40;
pub const KEY_GRAVE:      c_int = 41;
pub const KEY_LEFTSHIFT:  c_int = 42;
pub const KEY_BACKSLASH:  c_int = 43;
pub const KEY_Z:          c_int = 44;
pub const KEY_X:          c_int = 45;
pub const KEY_C:          c_int = 46;
pub const KEY_V:          c_int = 47;
pub const KEY_B:          c_int = 48;
pub const KEY_N:          c_int = 49;
pub const KEY_M:          c_int = 50;
pub const KEY_COMMA:      c_int = 51;
pub const KEY_DOT:        c_int = 52;
pub const KEY_SLASH:      c_int = 53;
pub const KEY_RIGHTSHIFT: c_int = 54;
pub const KEY_KPASTERISK: c_int = 55;
pub const KEY_LEFTALT:    c_int = 56;
pub const KEY_SPACE:      c_int = 57;
pub const KEY_CAPSLOCK:   c_int = 58;
pub const KEY_F1:         c_int = 59;
pub const KEY_F2:         c_int = 60;
pub const KEY_F3:         c_int = 61;
pub const KEY_F4:         c_int = 62;
pub const KEY_F5:         c_int = 63;
pub const KEY_F6:         c_int = 64;
pub const KEY_F7:         c_int = 65;
pub const KEY_F8:         c_int = 66;
pub const KEY_F9:         c_int = 67;
pub const KEY_F10:        c_int = 68;
pub const KEY_NUMLOCK:    c_int = 69;
pub const KEY_SCROLLLOCK: c_int = 70;
pub const KEY_KP7:        c_int = 71;
pub const KEY_KP8:        c_int = 72;
pub const KEY_KP9:        c_int = 73;
pub const KEY_KPMINUS:    c_int = 74;
pub const KEY_KP4:        c_int = 75;
pub const KEY_KP5:        c_int = 76;
pub const KEY_KP6:        c_int = 77;
pub const KEY_KPPLUS:     c_int = 78;
pub const KEY_KP1:        c_int = 79;
pub const KEY_KP2:        c_int = 80;
pub const KEY_KP3:        c_int = 81;
pub const KEY_KP0:        c_int = 82;
pub const KEY_KPDOT:      c_int = 83;

pub const KEY_ZENKAKUHANKAKU:   c_int = 85;
pub const KEY_102ND:            c_int = 86;
pub const KEY_F11:              c_int = 87;
pub const KEY_F12:              c_int = 88;
pub const KEY_RO:               c_int = 89;
pub const KEY_KATAKANA:         c_int = 90;
pub const KEY_HIRAGANA:         c_int = 91;
pub const KEY_HENKAN:           c_int = 92;
pub const KEY_KATAKANAHIRAGANA: c_int = 93;
pub const KEY_MUHENKAN:         c_int = 94;
pub const KEY_KPJPCOMMA:        c_int = 95;
pub const KEY_KPENTER:          c_int = 96;
pub const KEY_RIGHTCTRL:        c_int = 97;
pub const KEY_KPSLASH:          c_int = 98;
pub const KEY_SYSRQ:            c_int = 99;
pub const KEY_RIGHTALT:         c_int = 100;
pub const KEY_LINEFEED:         c_int = 101;
pub const KEY_HOME:             c_int = 102;
pub const KEY_UP:               c_int = 103;
pub const KEY_PAGEUP:           c_int = 104;
pub const KEY_LEFT:             c_int = 105;
pub const KEY_RIGHT:            c_int = 106;
pub const KEY_END:              c_int = 107;
pub const KEY_DOWN:             c_int = 108;
pub const KEY_PAGEDOWN:         c_int = 109;
pub const KEY_INSERT:           c_int = 110;
pub const KEY_DELETE:           c_int = 111;
pub const KEY_MACRO:            c_int = 112;
pub const KEY_MUTE:             c_int = 113;
pub const KEY_VOLUMEDOWN:       c_int = 114;
pub const KEY_VOLUMEUP:         c_int = 115;
pub const KEY_POWER:            c_int = 116; /* SC System Power Down */
pub const KEY_KPEQUAL:          c_int = 117;
pub const KEY_KPPLUSMINUS:      c_int = 118;
pub const KEY_PAUSE:            c_int = 119;
pub const KEY_SCALE:            c_int = 120; /* AL Compiz Scale : c_int = Expose */

pub const KEY_KPCOMMA:   c_int = 121;
pub const KEY_HANGEUL:   c_int = 122;
pub const KEY_HANGUEL:   c_int = KEY_HANGEUL;
pub const KEY_HANJA:     c_int = 123;
pub const KEY_YEN:       c_int = 124;
pub const KEY_LEFTMETA:  c_int = 125;
pub const KEY_RIGHTMETA: c_int = 126;
pub const KEY_COMPOSE:   c_int = 127;

pub const KEY_STOP:           c_int = 128; /* AC Stop */
pub const KEY_AGAIN:          c_int = 129;
pub const KEY_PROPS:          c_int = 130; /* AC Properties */
pub const KEY_UNDO:           c_int = 131; /* AC Undo */
pub const KEY_FRONT:          c_int = 132;
pub const KEY_COPY:           c_int = 133; /* AC Copy */
pub const KEY_OPEN:           c_int = 134; /* AC Open */
pub const KEY_PASTE:          c_int = 135; /* AC Paste */
pub const KEY_FIND:           c_int = 136; /* AC Search */
pub const KEY_CUT:            c_int = 137; /* AC Cut */
pub const KEY_HELP:           c_int = 138; /* AL Integrated Help Center */
pub const KEY_MENU:           c_int = 139; /* Menu : c_int = show menu */
pub const KEY_CALC:           c_int = 140; /* AL Calculator */
pub const KEY_SETUP:          c_int = 141;
pub const KEY_SLEEP:          c_int = 142; /* SC System Sleep */
pub const KEY_WAKEUP:         c_int = 143; /* System Wake Up */
pub const KEY_FILE:           c_int = 144; /* AL Local Machine Browser */
pub const KEY_SENDFILE:       c_int = 145;
pub const KEY_DELETEFILE:     c_int = 146;
pub const KEY_XFER:           c_int = 147;
pub const KEY_PROG1:          c_int = 148;
pub const KEY_PROG2:          c_int = 149;
pub const KEY_WWW:            c_int = 150; /* AL Internet Browser */
pub const KEY_MSDOS:          c_int = 151;
pub const KEY_COFFEE:         c_int = 152; /* AL Terminal Lock/Screensaver */
pub const KEY_SCREENLOCK:     c_int = KEY_COFFEE;
pub const KEY_ROTATE_DISPLAY: c_int = 153; /* Display orientation for e.g. tablets */
pub const KEY_DIRECTION:      c_int = KEY_ROTATE_DISPLAY;
pub const KEY_CYCLEWINDOWS:   c_int = 154;
pub const KEY_MAIL:           c_int = 155;
pub const KEY_BOOKMARKS:      c_int = 156; /* AC Bookmarks */
pub const KEY_COMPUTER:       c_int = 157;
pub const KEY_BACK:           c_int = 158; /* AC Back */
pub const KEY_FORWARD:        c_int = 159; /* AC Forward */
pub const KEY_CLOSECD:        c_int = 160;
pub const KEY_EJECTCD:        c_int = 161;
pub const KEY_EJECTCLOSECD:   c_int = 162;
pub const KEY_NEXTSONG:       c_int = 163;
pub const KEY_PLAYPAUSE:      c_int = 164;
pub const KEY_PREVIOUSSONG:   c_int = 165;
pub const KEY_STOPCD:         c_int = 166;
pub const KEY_RECORD:         c_int = 167;
pub const KEY_REWIND:         c_int = 168;
pub const KEY_PHONE:          c_int = 169; /* Media Select Telephone */
pub const KEY_ISO:            c_int = 170;
pub const KEY_CONFIG:         c_int = 171; /* AL Consumer Control Configuration */
pub const KEY_HOMEPAGE:       c_int = 172; /* AC Home */
pub const KEY_REFRESH:        c_int = 173; /* AC Refresh */
pub const KEY_EXIT:           c_int = 174; /* AC Exit */
pub const KEY_MOVE:           c_int = 175;
pub const KEY_EDIT:           c_int = 176;
pub const KEY_SCROLLUP:       c_int = 177;
pub const KEY_SCROLLDOWN:     c_int = 178;
pub const KEY_KPLEFTPAREN:    c_int = 179;
pub const KEY_KPRIGHTPAREN:   c_int = 180;
pub const KEY_NEW:            c_int = 181; /* AC New */
pub const KEY_REDO:           c_int = 182; /* AC Redo/Repeat */

pub const KEY_F13: c_int = 183;
pub const KEY_F14: c_int = 184;
pub const KEY_F15: c_int = 185;
pub const KEY_F16: c_int = 186;
pub const KEY_F17: c_int = 187;
pub const KEY_F18: c_int = 188;
pub const KEY_F19: c_int = 189;
pub const KEY_F20: c_int = 190;
pub const KEY_F21: c_int = 191;
pub const KEY_F22: c_int = 192;
pub const KEY_F23: c_int = 193;
pub const KEY_F24: c_int = 194;

pub const KEY_PLAYCD:         c_int = 200;
pub const KEY_PAUSECD:        c_int = 201;
pub const KEY_PROG3:          c_int = 202;
pub const KEY_PROG4:          c_int = 203;
pub const KEY_DASHBOARD:      c_int = 204; /* AL Dashboard */
pub const KEY_SUSPEND:        c_int = 205;
pub const KEY_CLOSE:          c_int = 206; /* AC Close */
pub const KEY_PLAY:           c_int = 207;
pub const KEY_FASTFORWARD:    c_int = 208;
pub const KEY_BASSBOOST:      c_int = 209;
pub const KEY_PRINT:          c_int = 210; /* AC Print */
pub const KEY_HP:             c_int = 211;
pub const KEY_CAMERA:         c_int = 212;
pub const KEY_SOUND:          c_int = 213;
pub const KEY_QUESTION:       c_int = 214;
pub const KEY_EMAIL:          c_int = 215;
pub const KEY_CHAT:           c_int = 216;
pub const KEY_SEARCH:         c_int = 217;
pub const KEY_CONNECT:        c_int = 218;
pub const KEY_FINANCE:        c_int = 219; /* AL Checkbook/Finance */
pub const KEY_SPORT:          c_int = 220;
pub const KEY_SHOP:           c_int = 221;
pub const KEY_ALTERASE:       c_int = 222;
pub const KEY_CANCEL:         c_int = 223; /* AC Cancel */
pub const KEY_BRIGHTNESSDOWN: c_int = 224;
pub const KEY_BRIGHTNESSUP:   c_int = 225;
pub const KEY_MEDIA:          c_int = 226;

pub const KEY_SWITCHVIDEOMODE: c_int = 227; /* Cycle between available video outputs (Monitor/LCD/TV-out/etc) */
pub const KEY_KBDILLUMTOGGLE:  c_int = 228;
pub const KEY_KBDILLUMDOWN:    c_int = 229;
pub const KEY_KBDILLUMUP:      c_int = 230;

pub const KEY_SEND:        c_int = 231; /* AC Send */
pub const KEY_REPLY:       c_int = 232; /* AC Reply */
pub const KEY_FORWARDMAIL: c_int = 233; /* AC Forward Msg */
pub const KEY_SAVE:        c_int = 234; /* AC Save */
pub const KEY_DOCUMENTS:   c_int = 235;

pub const KEY_BATTERY: c_int = 236;

pub const KEY_BLUETOOTH: c_int = 237;
pub const KEY_WLAN:      c_int = 238;
pub const KEY_UWB:       c_int = 239;

pub const KEY_UNKNOWN: c_int = 240;

pub const KEY_VIDEO_NEXT:       c_int = 241; /* drive next video source */
pub const KEY_VIDEO_PREV:       c_int = 242; /* drive previous video source */
pub const KEY_BRIGHTNESS_CYCLE: c_int = 243; /* brightness up, after max is min */
pub const KEY_BRIGHTNESS_AUTO:  c_int = 244; /* Set Auto Brightness: manual brightness control is off, rely on ambient */
pub const KEY_BRIGHTNESS_ZERO:  c_int = KEY_BRIGHTNESS_AUTO;
pub const KEY_DISPLAY_OFF:      c_int = 245; /* display device to off state */

pub const KEY_WWAN:   c_int = 246; /* Wireless WAN : c_int = LTE, UMTS, GSM, etc. */
pub const KEY_WIMAX:  c_int = KEY_WWAN;
pub const KEY_RFKILL: c_int = 247; /* Key that controls all radios */

pub const KEY_MICMUTE: c_int = 248; /* Mute / unmute the microphone */

/* Code 255 is reserved for special needs of AT keyboard driver */

pub const BTN_MISC: c_int = 0x100;
pub const BTN_0:    c_int = 0x100;
pub const BTN_1:    c_int = 0x101;
pub const BTN_2:    c_int = 0x102;
pub const BTN_3:    c_int = 0x103;
pub const BTN_4:    c_int = 0x104;
pub const BTN_5:    c_int = 0x105;
pub const BTN_6:    c_int = 0x106;
pub const BTN_7:    c_int = 0x107;
pub const BTN_8:    c_int = 0x108;
pub const BTN_9:    c_int = 0x109;

pub const BTN_MOUSE:   c_int = 0x110;
pub const BTN_LEFT:    c_int = 0x110;
pub const BTN_RIGHT:   c_int = 0x111;
pub const BTN_MIDDLE:  c_int = 0x112;
pub const BTN_SIDE:    c_int = 0x113;
pub const BTN_EXTRA:   c_int = 0x114;
pub const BTN_FORWARD: c_int = 0x115;
pub const BTN_BACK:    c_int = 0x116;
pub const BTN_TASK:    c_int = 0x117;

pub const BTN_JOYSTICK: c_int = 0x120;
pub const BTN_TRIGGER:  c_int = 0x120;
pub const BTN_THUMB:    c_int = 0x121;
pub const BTN_THUMB2:   c_int = 0x122;
pub const BTN_TOP:      c_int = 0x123;
pub const BTN_TOP2:     c_int = 0x124;
pub const BTN_PINKIE:   c_int = 0x125;
pub const BTN_BASE:     c_int = 0x126;
pub const BTN_BASE2:    c_int = 0x127;
pub const BTN_BASE3:    c_int = 0x128;
pub const BTN_BASE4:    c_int = 0x129;
pub const BTN_BASE5:    c_int = 0x12a;
pub const BTN_BASE6:    c_int = 0x12b;
pub const BTN_DEAD:     c_int = 0x12f;

pub const BTN_GAMEPAD: c_int = 0x130;
pub const BTN_SOUTH:   c_int = 0x130;
pub const BTN_A:       c_int = BTN_SOUTH;
pub const BTN_EAST:    c_int = 0x131;
pub const BTN_B:       c_int = BTN_EAST;
pub const BTN_C:       c_int = 0x132;
pub const BTN_NORTH:   c_int = 0x133;
pub const BTN_X:       c_int = BTN_NORTH;
pub const BTN_WEST:    c_int = 0x134;
pub const BTN_Y:       c_int = BTN_WEST;
pub const BTN_Z:       c_int = 0x135;
pub const BTN_TL:      c_int = 0x136;
pub const BTN_TR:      c_int = 0x137;
pub const BTN_TL2:     c_int = 0x138;
pub const BTN_TR2:     c_int = 0x139;
pub const BTN_SELECT:  c_int = 0x13a;
pub const BTN_START:   c_int = 0x13b;
pub const BTN_MODE:    c_int = 0x13c;
pub const BTN_THUMBL:  c_int = 0x13d;
pub const BTN_THUMBR:  c_int = 0x13e;

pub const BTN_DIGI:           c_int = 0x140;
pub const BTN_TOOL_PEN:       c_int = 0x140;
pub const BTN_TOOL_RUBBER:    c_int = 0x141;
pub const BTN_TOOL_BRUSH:     c_int = 0x142;
pub const BTN_TOOL_PENCIL:    c_int = 0x143;
pub const BTN_TOOL_AIRBRUSH:  c_int = 0x144;
pub const BTN_TOOL_FINGER:    c_int = 0x145;
pub const BTN_TOOL_MOUSE:     c_int = 0x146;
pub const BTN_TOOL_LENS:      c_int = 0x147;
pub const BTN_TOOL_QUINTTAP:  c_int = 0x148; /* Five fingers on trackpad */
pub const BTN_TOUCH:          c_int = 0x14a;
pub const BTN_STYLUS:         c_int = 0x14b;
pub const BTN_STYLUS2:        c_int = 0x14c;
pub const BTN_TOOL_DOUBLETAP: c_int = 0x14d;
pub const BTN_TOOL_TRIPLETAP: c_int = 0x14e;
pub const BTN_TOOL_QUADTAP:   c_int = 0x14f; /* Four fingers on trackpad */

pub const BTN_WHEEL:     c_int = 0x150;
pub const BTN_GEAR_DOWN: c_int = 0x150;
pub const BTN_GEAR_UP:   c_int = 0x151;

pub const KEY_OK:                c_int = 0x160;
pub const KEY_SELECT:            c_int = 0x161;
pub const KEY_GOTO:              c_int = 0x162;
pub const KEY_CLEAR:             c_int = 0x163;
pub const KEY_POWER2:            c_int = 0x164;
pub const KEY_OPTION:            c_int = 0x165;
pub const KEY_INFO:              c_int = 0x166; /* AL OEM Features/Tips/Tutorial */
pub const KEY_TIME:              c_int = 0x167;
pub const KEY_VENDOR:            c_int = 0x168;
pub const KEY_ARCHIVE:           c_int = 0x169;
pub const KEY_PROGRAM:           c_int = 0x16a; /* Media Select Program Guide */
pub const KEY_CHANNEL:           c_int = 0x16b;
pub const KEY_FAVORITES:         c_int = 0x16c;
pub const KEY_EPG:               c_int = 0x16d;
pub const KEY_PVR:               c_int = 0x16e; /* Media Select Home */
pub const KEY_MHP:               c_int = 0x16f;
pub const KEY_LANGUAGE:          c_int = 0x170;
pub const KEY_TITLE:             c_int = 0x171;
pub const KEY_SUBTITLE:          c_int = 0x172;
pub const KEY_ANGLE:             c_int = 0x173;
pub const KEY_ZOOM:              c_int = 0x174;
pub const KEY_MODE:              c_int = 0x175;
pub const KEY_KEYBOARD:          c_int = 0x176;
pub const KEY_SCREEN:            c_int = 0x177;
pub const KEY_PC:                c_int = 0x178; /* Media Select Computer */
pub const KEY_TV:                c_int = 0x179; /* Media Select TV */
pub const KEY_TV2:               c_int = 0x17a; /* Media Select Cable */
pub const KEY_VCR:               c_int = 0x17b; /* Media Select VCR */
pub const KEY_VCR2:              c_int = 0x17c; /* VCR Plus */
pub const KEY_SAT:               c_int = 0x17d; /* Media Select Satellite */
pub const KEY_SAT2:              c_int = 0x17e;
pub const KEY_CD:                c_int = 0x17f; /* Media Select CD */
pub const KEY_TAPE:              c_int = 0x180; /* Media Select Tape */
pub const KEY_RADIO:             c_int = 0x181;
pub const KEY_TUNER:             c_int = 0x182; /* Media Select Tuner */
pub const KEY_PLAYER:            c_int = 0x183;
pub const KEY_TEXT:              c_int = 0x184;
pub const KEY_DVD:               c_int = 0x185; /* Media Select DVD */
pub const KEY_AUX:               c_int = 0x186;
pub const KEY_MP3:               c_int = 0x187;
pub const KEY_AUDIO:             c_int = 0x188; /* AL Audio Browser */
pub const KEY_VIDEO:             c_int = 0x189; /* AL Movie Browser */
pub const KEY_DIRECTORY:         c_int = 0x18a;
pub const KEY_LIST:              c_int = 0x18b;
pub const KEY_MEMO:              c_int = 0x18c; /* Media Select Messages */
pub const KEY_CALENDAR:          c_int = 0x18d;
pub const KEY_RED:               c_int = 0x18e;
pub const KEY_GREEN:             c_int = 0x18f;
pub const KEY_YELLOW:            c_int = 0x190;
pub const KEY_BLUE:              c_int = 0x191;
pub const KEY_CHANNELUP:         c_int = 0x192; /* Channel Increment */
pub const KEY_CHANNELDOWN:       c_int = 0x193; /* Channel Decrement */
pub const KEY_FIRST:             c_int = 0x194;
pub const KEY_LAST:              c_int = 0x195; /* Recall Last */
pub const KEY_AB:                c_int = 0x196;
pub const KEY_NEXT:              c_int = 0x197;
pub const KEY_RESTART:           c_int = 0x198;
pub const KEY_SLOW:              c_int = 0x199;
pub const KEY_SHUFFLE:           c_int = 0x19a;
pub const KEY_BREAK:             c_int = 0x19b;
pub const KEY_PREVIOUS:          c_int = 0x19c;
pub const KEY_DIGITS:            c_int = 0x19d;
pub const KEY_TEEN:              c_int = 0x19e;
pub const KEY_TWEN:              c_int = 0x19f;
pub const KEY_VIDEOPHONE:        c_int = 0x1a0; /* Media Select Video Phone */
pub const KEY_GAMES:             c_int = 0x1a1; /* Media Select Games */
pub const KEY_ZOOMIN:            c_int = 0x1a2; /* AC Zoom In */
pub const KEY_ZOOMOUT:           c_int = 0x1a3; /* AC Zoom Out */
pub const KEY_ZOOMRESET:         c_int = 0x1a4; /* AC Zoom */
pub const KEY_WORDPROCESSOR:     c_int = 0x1a5; /* AL Word Processor */
pub const KEY_EDITOR:            c_int = 0x1a6; /* AL Text Editor */
pub const KEY_SPREADSHEET:       c_int = 0x1a7; /* AL Spreadsheet */
pub const KEY_GRAPHICSEDITOR:    c_int = 0x1a8; /* AL Graphics Editor */
pub const KEY_PRESENTATION:      c_int = 0x1a9; /* AL Presentation App */
pub const KEY_DATABASE:          c_int = 0x1aa; /* AL Database App */
pub const KEY_NEWS:              c_int = 0x1ab; /* AL Newsreader */
pub const KEY_VOICEMAIL:         c_int = 0x1ac; /* AL Voicemail */
pub const KEY_ADDRESSBOOK:       c_int = 0x1ad; /* AL Contacts/Address Book */
pub const KEY_MESSENGER:         c_int = 0x1ae; /* AL Instant Messaging */
pub const KEY_DISPLAYTOGGLE:     c_int = 0x1af; /* Turn display : c_int = LCD on and off */
pub const KEY_BRIGHTNESS_TOGGLE: c_int = KEY_DISPLAYTOGGLE;
pub const KEY_SPELLCHECK:        c_int = 0x1b0; /* AL Spell Check */
pub const KEY_LOGOFF:            c_int = 0x1b1; /* AL Logoff */

pub const KEY_DOLLAR: c_int = 0x1b2;
pub const KEY_EURO:   c_int = 0x1b3;

pub const KEY_FRAMEBACK:      c_int = 0x1b4; /* Consumer - transport controls */
pub const KEY_FRAMEFORWARD:   c_int = 0x1b5;
pub const KEY_CONTEXT_MENU:   c_int = 0x1b6; /* GenDesc - system context menu */
pub const KEY_MEDIA_REPEAT:   c_int = 0x1b7; /* Consumer - transport control */
pub const KEY_10CHANNELSUP:   c_int = 0x1b8; /* 10 channels up : c_int = 10+ */
pub const KEY_10CHANNELSDOWN: c_int = 0x1b9; /* 10 channels down : c_int = 10- */
pub const KEY_IMAGES:         c_int = 0x1ba; /* AL Image Browser */

pub const KEY_DEL_EOL:  c_int = 0x1c0;
pub const KEY_DEL_EOS:  c_int = 0x1c1;
pub const KEY_INS_LINE: c_int = 0x1c2;
pub const KEY_DEL_LINE: c_int = 0x1c3;

pub const KEY_FN:     c_int = 0x1d0;
pub const KEY_FN_ESC: c_int = 0x1d1;
pub const KEY_FN_F1:  c_int = 0x1d2;
pub const KEY_FN_F2:  c_int = 0x1d3;
pub const KEY_FN_F3:  c_int = 0x1d4;
pub const KEY_FN_F4:  c_int = 0x1d5;
pub const KEY_FN_F5:  c_int = 0x1d6;
pub const KEY_FN_F6:  c_int = 0x1d7;
pub const KEY_FN_F7:  c_int = 0x1d8;
pub const KEY_FN_F8:  c_int = 0x1d9;
pub const KEY_FN_F9:  c_int = 0x1da;
pub const KEY_FN_F10: c_int = 0x1db;
pub const KEY_FN_F11: c_int = 0x1dc;
pub const KEY_FN_F12: c_int = 0x1dd;
pub const KEY_FN_1:   c_int = 0x1de;
pub const KEY_FN_2:   c_int = 0x1df;
pub const KEY_FN_D:   c_int = 0x1e0;
pub const KEY_FN_E:   c_int = 0x1e1;
pub const KEY_FN_F:   c_int = 0x1e2;
pub const KEY_FN_S:   c_int = 0x1e3;
pub const KEY_FN_B:   c_int = 0x1e4;

pub const KEY_BRL_DOT1:  c_int = 0x1f1;
pub const KEY_BRL_DOT2:  c_int = 0x1f2;
pub const KEY_BRL_DOT3:  c_int = 0x1f3;
pub const KEY_BRL_DOT4:  c_int = 0x1f4;
pub const KEY_BRL_DOT5:  c_int = 0x1f5;
pub const KEY_BRL_DOT6:  c_int = 0x1f6;
pub const KEY_BRL_DOT7:  c_int = 0x1f7;
pub const KEY_BRL_DOT8:  c_int = 0x1f8;
pub const KEY_BRL_DOT9:  c_int = 0x1f9;
pub const KEY_BRL_DOT10: c_int = 0x1fa;

pub const KEY_NUMERIC_0:     c_int = 0x200; /* used by phones, remote controls, */
pub const KEY_NUMERIC_1:     c_int = 0x201; /* and other keypads */
pub const KEY_NUMERIC_2:     c_int = 0x202;
pub const KEY_NUMERIC_3:     c_int = 0x203;
pub const KEY_NUMERIC_4:     c_int = 0x204;
pub const KEY_NUMERIC_5:     c_int = 0x205;
pub const KEY_NUMERIC_6:     c_int = 0x206;
pub const KEY_NUMERIC_7:     c_int = 0x207;
pub const KEY_NUMERIC_8:     c_int = 0x208;
pub const KEY_NUMERIC_9:     c_int = 0x209;
pub const KEY_NUMERIC_STAR:  c_int = 0x20a;
pub const KEY_NUMERIC_POUND: c_int = 0x20b;
pub const KEY_NUMERIC_A:     c_int = 0x20c; /* Phone key A - HUT Telephony 0xb9 */
pub const KEY_NUMERIC_B:     c_int = 0x20d;
pub const KEY_NUMERIC_C:     c_int = 0x20e;
pub const KEY_NUMERIC_D:     c_int = 0x20f;

pub const KEY_CAMERA_FOCUS: c_int = 0x210;
pub const KEY_WPS_BUTTON:   c_int = 0x211; /* WiFi Protected Setup key */

pub const KEY_TOUCHPAD_TOGGLE: c_int = 0x212; /* Request switch touchpad on or off */
pub const KEY_TOUCHPAD_ON:     c_int = 0x213;
pub const KEY_TOUCHPAD_OFF:    c_int = 0x214;

pub const KEY_CAMERA_ZOOMIN:  c_int = 0x215;
pub const KEY_CAMERA_ZOOMOUT: c_int = 0x216;
pub const KEY_CAMERA_UP:      c_int = 0x217;
pub const KEY_CAMERA_DOWN:    c_int = 0x218;
pub const KEY_CAMERA_LEFT:    c_int = 0x219;
pub const KEY_CAMERA_RIGHT:   c_int = 0x21a;

pub const KEY_ATTENDANT_ON:     c_int = 0x21b;
pub const KEY_ATTENDANT_OFF:    c_int = 0x21c;
pub const KEY_ATTENDANT_TOGGLE: c_int = 0x21d; /* Attendant call on or off */
pub const KEY_LIGHTS_TOGGLE:    c_int = 0x21e; /* Reading light on or off */

pub const BTN_DPAD_UP:    c_int = 0x220;
pub const BTN_DPAD_DOWN:  c_int = 0x221;
pub const BTN_DPAD_LEFT:  c_int = 0x222;
pub const BTN_DPAD_RIGHT: c_int = 0x223;

pub const KEY_ALS_TOGGLE: c_int = 0x230; /* Ambient light sensor */

pub const KEY_BUTTONCONFIG: c_int = 0x240;  /* AL Button Configuration */
pub const KEY_TASKMANAGER:  c_int = 0x241;  /* AL Task/Project Manager */
pub const KEY_JOURNAL:      c_int = 0x242;  /* AL Log/Journal/Timecard */
pub const KEY_CONTROLPANEL: c_int = 0x243;  /* AL Control Panel */
pub const KEY_APPSELECT:    c_int = 0x244;  /* AL Select Task/Application */
pub const KEY_SCREENSAVER:  c_int = 0x245;  /* AL Screen Saver */
pub const KEY_VOICECOMMAND: c_int = 0x246;  /* Listening Voice Command */

pub const KEY_BRIGHTNESS_MIN: c_int = 0x250; /* Set Brightness to Minimum */
pub const KEY_BRIGHTNESS_MAX: c_int = 0x251; /* Set Brightness to Maximum */

pub const KEY_KBDINPUTASSIST_PREV:      c_int = 0x260;
pub const KEY_KBDINPUTASSIST_NEXT:      c_int = 0x261;
pub const KEY_KBDINPUTASSIST_PREVGROUP: c_int = 0x262;
pub const KEY_KBDINPUTASSIST_NEXTGROUP: c_int = 0x263;
pub const KEY_KBDINPUTASSIST_ACCEPT:    c_int = 0x264;
pub const KEY_KBDINPUTASSIST_CANCEL:    c_int = 0x265;

pub const BTN_TRIGGER_HAPPY:   c_int = 0x2c0;
pub const BTN_TRIGGER_HAPPY1:  c_int = 0x2c0;
pub const BTN_TRIGGER_HAPPY2:  c_int = 0x2c1;
pub const BTN_TRIGGER_HAPPY3:  c_int = 0x2c2;
pub const BTN_TRIGGER_HAPPY4:  c_int = 0x2c3;
pub const BTN_TRIGGER_HAPPY5:  c_int = 0x2c4;
pub const BTN_TRIGGER_HAPPY6:  c_int = 0x2c5;
pub const BTN_TRIGGER_HAPPY7:  c_int = 0x2c6;
pub const BTN_TRIGGER_HAPPY8:  c_int = 0x2c7;
pub const BTN_TRIGGER_HAPPY9:  c_int = 0x2c8;
pub const BTN_TRIGGER_HAPPY10: c_int = 0x2c9;
pub const BTN_TRIGGER_HAPPY11: c_int = 0x2ca;
pub const BTN_TRIGGER_HAPPY12: c_int = 0x2cb;
pub const BTN_TRIGGER_HAPPY13: c_int = 0x2cc;
pub const BTN_TRIGGER_HAPPY14: c_int = 0x2cd;
pub const BTN_TRIGGER_HAPPY15: c_int = 0x2ce;
pub const BTN_TRIGGER_HAPPY16: c_int = 0x2cf;
pub const BTN_TRIGGER_HAPPY17: c_int = 0x2d0;
pub const BTN_TRIGGER_HAPPY18: c_int = 0x2d1;
pub const BTN_TRIGGER_HAPPY19: c_int = 0x2d2;
pub const BTN_TRIGGER_HAPPY20: c_int = 0x2d3;
pub const BTN_TRIGGER_HAPPY21: c_int = 0x2d4;
pub const BTN_TRIGGER_HAPPY22: c_int = 0x2d5;
pub const BTN_TRIGGER_HAPPY23: c_int = 0x2d6;
pub const BTN_TRIGGER_HAPPY24: c_int = 0x2d7;
pub const BTN_TRIGGER_HAPPY25: c_int = 0x2d8;
pub const BTN_TRIGGER_HAPPY26: c_int = 0x2d9;
pub const BTN_TRIGGER_HAPPY27: c_int = 0x2da;
pub const BTN_TRIGGER_HAPPY28: c_int = 0x2db;
pub const BTN_TRIGGER_HAPPY29: c_int = 0x2dc;
pub const BTN_TRIGGER_HAPPY30: c_int = 0x2dd;
pub const BTN_TRIGGER_HAPPY31: c_int = 0x2de;
pub const BTN_TRIGGER_HAPPY32: c_int = 0x2df;
pub const BTN_TRIGGER_HAPPY33: c_int = 0x2e0;
pub const BTN_TRIGGER_HAPPY34: c_int = 0x2e1;
pub const BTN_TRIGGER_HAPPY35: c_int = 0x2e2;
pub const BTN_TRIGGER_HAPPY36: c_int = 0x2e3;
pub const BTN_TRIGGER_HAPPY37: c_int = 0x2e4;
pub const BTN_TRIGGER_HAPPY38: c_int = 0x2e5;
pub const BTN_TRIGGER_HAPPY39: c_int = 0x2e6;
pub const BTN_TRIGGER_HAPPY40: c_int = 0x2e7;

/* We avoid low common keys in module aliases so they don't get huge. */
pub const KEY_MIN_INTERESTING: c_int = KEY_MUTE;
pub const KEY_MAX:             c_int = 0x2ff;
pub const KEY_CNT:             c_int = KEY_MAX + 1;

/*
 * Relative axes
 */

pub const REL_X:      c_int = 0x00;
pub const REL_Y:      c_int = 0x01;
pub const REL_Z:      c_int = 0x02;
pub const REL_RX:     c_int = 0x03;
pub const REL_RY:     c_int = 0x04;
pub const REL_RZ:     c_int = 0x05;
pub const REL_HWHEEL: c_int = 0x06;
pub const REL_DIAL:   c_int = 0x07;
pub const REL_WHEEL:  c_int = 0x08;
pub const REL_MISC:   c_int = 0x09;
pub const REL_MAX:    c_int = 0x0f;
pub const REL_CNT:    c_int = REL_MAX + 1;

/*
 * Absolute axes
 */

pub const ABS_X:          c_int = 0x00;
pub const ABS_Y:          c_int = 0x01;
pub const ABS_Z:          c_int = 0x02;
pub const ABS_RX:         c_int = 0x03;
pub const ABS_RY:         c_int = 0x04;
pub const ABS_RZ:         c_int = 0x05;
pub const ABS_THROTTLE:   c_int = 0x06;
pub const ABS_RUDDER:     c_int = 0x07;
pub const ABS_WHEEL:      c_int = 0x08;
pub const ABS_GAS:        c_int = 0x09;
pub const ABS_BRAKE:      c_int = 0x0a;
pub const ABS_HAT0X:      c_int = 0x10;
pub const ABS_HAT0Y:      c_int = 0x11;
pub const ABS_HAT1X:      c_int = 0x12;
pub const ABS_HAT1Y:      c_int = 0x13;
pub const ABS_HAT2X:      c_int = 0x14;
pub const ABS_HAT2Y:      c_int = 0x15;
pub const ABS_HAT3X:      c_int = 0x16;
pub const ABS_HAT3Y:      c_int = 0x17;
pub const ABS_PRESSURE:   c_int = 0x18;
pub const ABS_DISTANCE:   c_int = 0x19;
pub const ABS_TILT_X:     c_int = 0x1a;
pub const ABS_TILT_Y:     c_int = 0x1b;
pub const ABS_TOOL_WIDTH: c_int = 0x1c;

pub const ABS_VOLUME: c_int = 0x20;

pub const ABS_MISC: c_int = 0x28;

pub const ABS_MT_SLOT:        c_int = 0x2f; /* MT slot being modified */
pub const ABS_MT_TOUCH_MAJOR: c_int = 0x30; /* Major axis of touching ellipse */
pub const ABS_MT_TOUCH_MINOR: c_int = 0x31; /* Minor axis : c_int = omit if circular */
pub const ABS_MT_WIDTH_MAJOR: c_int = 0x32; /* Major axis of approaching ellipse */
pub const ABS_MT_WIDTH_MINOR: c_int = 0x33; /* Minor axis : c_int = omit if circular */
pub const ABS_MT_ORIENTATION: c_int = 0x34; /* Ellipse orientation */
pub const ABS_MT_POSITION_X:  c_int = 0x35; /* Center X touch position */
pub const ABS_MT_POSITION_Y:  c_int = 0x36; /* Center Y touch position */
pub const ABS_MT_TOOL_TYPE:   c_int = 0x37; /* Type of touching device */
pub const ABS_MT_BLOB_ID:     c_int = 0x38; /* Group a set of packets as a blob */
pub const ABS_MT_TRACKING_ID: c_int = 0x39; /* Unique ID of initiated contact */
pub const ABS_MT_PRESSURE:    c_int = 0x3a; /* Pressure on contact area */
pub const ABS_MT_DISTANCE:    c_int = 0x3b; /* Contact hover distance */
pub const ABS_MT_TOOL_X:      c_int = 0x3c; /* Center X tool position */
pub const ABS_MT_TOOL_Y:      c_int = 0x3d; /* Center Y tool position */


pub const ABS_MAX: c_int = 0x3f;
pub const ABS_CNT: c_int = ABS_MAX + 1;

/*
 * Switch events
 */

pub const SW_LID:                  c_int = 0x00;  /* set = lid shut */
pub const SW_TABLET_MODE:          c_int = 0x01;  /* set = tablet mode */
pub const SW_HEADPHONE_INSERT:     c_int = 0x02;  /* set = inserted */
pub const SW_RFKILL_ALL:           c_int = 0x03;  /* rfkill master switch, type "any" set = radio enabled */
pub const SW_RADIO:                c_int = SW_RFKILL_ALL; /* deprecated */
pub const SW_MICROPHONE_INSERT:    c_int = 0x04;  /* set = inserted */
pub const SW_DOCK:                 c_int = 0x05;  /* set = plugged into dock */
pub const SW_LINEOUT_INSERT:       c_int = 0x06;  /* set = inserted */
pub const SW_JACK_PHYSICAL_INSERT: c_int = 0x07;  /* set = mechanical switch set */
pub const SW_VIDEOOUT_INSERT:      c_int = 0x08;  /* set = inserted */
pub const SW_CAMERA_LENS_COVER:    c_int = 0x09;  /* set = lens covered */
pub const SW_KEYPAD_SLIDE:         c_int = 0x0a;  /* set = keypad slide out */
pub const SW_FRONT_PROXIMITY:      c_int = 0x0b;  /* set = front proximity sensor active */
pub const SW_ROTATE_LOCK:          c_int = 0x0c;  /* set = rotate locked/disabled */
pub const SW_LINEIN_INSERT:        c_int = 0x0d;  /* set = inserted */
pub const SW_MUTE_DEVICE:          c_int = 0x0e;  /* set = device disabled */
pub const SW_MAX:                  c_int = 0x0f;
pub const SW_CNT:                  c_int = SW_MAX + 1;

/*
 * Misc events
 */

pub const MSC_SERIAL:    c_int = 0x00;
pub const MSC_PULSELED:  c_int = 0x01;
pub const MSC_GESTURE:   c_int = 0x02;
pub const MSC_RAW:       c_int = 0x03;
pub const MSC_SCAN:      c_int = 0x04;
pub const MSC_TIMESTAMP: c_int = 0x05;
pub const MSC_MAX:       c_int = 0x07;
pub const MSC_CNT:       c_int = MSC_MAX + 1;

/*
 * LEDs
 */

pub const LED_NUML:     c_int = 0x00;
pub const LED_CAPSL:    c_int = 0x01;
pub const LED_SCROLLL:  c_int = 0x02;
pub const LED_COMPOSE:  c_int = 0x03;
pub const LED_KANA:     c_int = 0x04;
pub const LED_SLEEP:    c_int = 0x05;
pub const LED_SUSPEND:  c_int = 0x06;
pub const LED_MUTE:     c_int = 0x07;
pub const LED_MISC:     c_int = 0x08;
pub const LED_MAIL:     c_int = 0x09;
pub const LED_CHARGING: c_int = 0x0a;
pub const LED_MAX:      c_int = 0x0f;
pub const LED_CNT:      c_int = LED_MAX + 1;

/*
 * Autorepeat values
 */

pub const REP_DELAY:  c_int = 0x00;
pub const REP_PERIOD: c_int = 0x01;
pub const REP_MAX:    c_int = 0x01;
pub const REP_CNT:    c_int = REP_MAX + 1;

/*
 * Sounds
 */

pub const SND_CLICK: c_int = 0x00;
pub const SND_BELL:  c_int = 0x01;
pub const SND_TONE:  c_int = 0x02;
pub const SND_MAX:   c_int = 0x07;
pub const SND_CNT:   c_int = SND_MAX + 1;
