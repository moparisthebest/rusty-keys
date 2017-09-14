#!/usr/bin/env python
# pressing all of these keys along with a number key representing the index of keymaps changes the layout
# ie, in this case pressing both and 0 would go QWERTY, while both and 1 would go dvorak
switch_layout_keys = ['LEFTSHIFT','RIGHTSHIFT']

# pressing QWERTY reverts to the index specified in revert_keymap_index for only the duration of the pressing
# used so QWERTY shortcuts like Ctrl+C still work
revert_default_key = 'LEFTCTRL'
revert_keymap_index = 0

# this is the default index to use when the program first starts
# in this case, 2 means modified Progammer Dvorak
default_keymap_index = 2

# these are keys that caps_lock doesn't modify by default, but that you would like it to, affects all keymaps
caps_lock_modify = """
    GRV, 1,   2,   3,   4,   5,   6,   7,   8,   9,   0,   MINS,EQL, BSPC,                             PSLS,PAST,PMNS,
                                                           LBRC,RBRC,BSLS,                        P7,  P8,  P9,
                                                      SCLN,QUOT,                                  P4,  P5,  P6,  PPLS,
                                            COMM,DOT, SLSH,                                       P1,  P2,  P3,
                                                                                                  P0,       PDOT
    """

# these are the keymaps available, you can add as many as you want or re-order them, just be aware the mapping is
# always done from the first one to all subsequent ones, so you probably want to leave QWERTY or similar up top
keymaps = [
    # default key layout, QWERTY in this case
    """
    ESC, F1,  F2,  F3,  F4,  F5,  F6,  F7,  F8,  F9,  F10, F11, F12,           PSCR,SLCK,BRK,
    GRV, 1,   2,   3,   4,   5,   6,   7,   8,   9,   0,   MINS,EQL, BSPC,     INS, HOME,PGUP,    NLCK,PSLS,PAST,PMNS,
    TAB, Q,   W,   E,   R,   T,   Y,   U,   I,   O,   P,   LBRC,RBRC,BSLS,     DEL, END, PGDN,    P7,  P8,  P9,
    CAPS,A,   S,   D,   F,   G,   H,   J,   K,   L,   SCLN,QUOT,     ENT,                         P4,  P5,  P6,  PPLS,
    LSFT,Z,   X,   C,   V,   B,   N,   M,   COMM,DOT, SLSH,          RSFT,          UP,           P1,  P2,  P3,
    LCTL,LGUI,LALT,          SPC,                     RALT,RGUI,APP, RCTL,     LEFT,DOWN,RGHT,    P0,       PDOT,PENT
    """,
    # Dvorak http://en.wikipedia.org/wiki/Dvorak_Simplified_Keyboard
    # https://www.moparisthebest.com/kbs/standard-dvorak-QwertySecondary.svg
    """
    ESC, F1,  F2,  F3,  F4,  F5,  F6,  F7,  F8,  F9,  F10, F11, F12,           PSCR,SLCK,BRK,
    GRV, 1,   2,   3,   4,   5,   6,   7,   8,   9,   0,   LBRC,RBRC,BSPC,     INS, HOME,PGUP,    NLCK,PSLS,PAST,PMNS,
    TAB, QUOT,COMM,DOT, P,   Y,   F,   G,   C,   R,   L,   SLSH,EQL, BSLS,     DEL, END, PGDN,    P7,  P8,  P9,
    CAPS,A,   O,   E,   U,   I,   D,   H,   T,   N,   S,   MINS,     ENT,                         P4,  P5,  P6,  PPLS,
    LSFT,SCLN,Q,   J,   K,   X,   B,   M,   W,   V,   Z,             RSFT,          UP,           P1,  P2,  P3,
    LCTL,LGUI,LALT,          SPC,                     RALT,RGUI,APP, RCTL,     LEFT,DOWN,RGHT,    P0,       PDOT,PENT
    """,
    # Unix Dvorak Programmer Dvorak - for unix developers who are switching from dvorak
    # https://www.moparisthebest.com/kbs/programmer-dvorak-NoSecondary-NumpadStandard-NoSwap-StandardNums-SwapAt-SwapPipe.svg
    """
    ESC,      F1,    F2,     F3,      F4,      F5,   F6,    F7,   F8,   F9,     F10,    F11,   F12,                 PSCR,SLCK,BRK,
    ^4:^GRV,  ^7:1,  LBRC:2, ^LBRC:3, ^RBRC:4, ^9:5, ^2:6,  ^8:7, ^0:8, ^EQL:9, RBRC:0, ^1:^5, ^3:GRV, BSPC,        INS, HOME,PGUP,    NLCK,   PSLS:^9, PAST:^0,   PMNS:^4,
    TAB,      QUOT,  COMM,   DOT,     P,       Y,    F,     G,    C,    R,      L,      SLSH,  EQL:^6, ^BSLS:BSLS,  DEL, END, PGDN,    P7:^A,  P8:^B,   P9:^C,
    CAPS,     A,     O,      E,       U,       I,    D,     H,    T,    N,      S,      MINS,          ENT,                            P4:^D,  P5:^E,   P6:^F,     PPLS:COMM,
    LSFT,     SCLN,  Q,      J,       K,       X,    B,     M,    W,    V,      Z,                     RSFT,             UP,           P1:EQL, P2:X,    P3:^SCLN,
    LCTL,     LGUI,  LALT,                     SPC,                             RALT,   RGUI,  APP,    RCTL,        LEFT,DOWN,RGHT,    P0:BSLS,         PDOT:SCLN, PENT
    """,
    # Unix Dvorak Programmer Dvorak - for unix developers who are switching from dvorak - phone numpad
    # https://www.moparisthebest.com/kbs/programmer-dvorak-QwertySecondary-NumpadPhone-NoSwap-StandardNums-SwapAt-SwapPipe.svg
    """
    ESC,      F1,    F2,     F3,      F4,      F5,   F6,    F7,   F8,   F9,     F10,    F11,   F12,                 PSCR,SLCK,BRK,
    ^4:^GRV,  ^7:1,  LBRC:2, ^LBRC:3, ^RBRC:4, ^9:5, ^2:6,  ^8:7, ^0:8, ^EQL:9, RBRC:0, ^1:^5, ^3:GRV, BSPC,        INS, HOME,PGUP,    NLCK,   PSLS:^9, PAST:^0,   PMNS:^4,
    TAB,      QUOT,  COMM,   DOT,     P,       Y,    F,     G,    C,    R,      L,      SLSH,  EQL:^6, ^BSLS:BSLS,  DEL, END, PGDN,    P1:^A,  P2:^B,   P3:^C,
    CAPS,     A,     O,      E,       U,       I,    D,     H,    T,    N,      S,      MINS,          ENT,                            P4:^D,  P5:^E,   P6:^F,     PPLS:COMM,
    LSFT,     SCLN,  Q,      J,       K,       X,    B,     M,    W,    V,      Z,                     RSFT,             UP,           P7:EQL, P8:X,    P9:^SCLN,
    LCTL,     LGUI,  LALT,                     SPC,                             RALT,   RGUI,  APP,    RCTL,        LEFT,DOWN,RGHT,    P0:BSLS,         PDOT:SCLN, PENT
    """,
    # Programmer Dvorak http://www.kaufmann.no/roland/dvorak/
    # https://www.moparisthebest.com/kbs/programmer-dvorak-QwertySecondary-NumpadPhone-StrictSwap-StrictNums-StrictAt-StrictPipe.svg
    """
    ESC,      F1,    F2,     F3,      F4,      F5,   F6,    F7,   F8,   F9,     F10,    F11,   F12,                 PSCR,SLCK,BRK,
    ^4:^GRV,  ^7:^5, LBRC:7, ^LBRC:5, ^RBRC:3, ^9:1, EQL:9, ^8:0, ^0:2, ^EQL:4, RBRC:6, ^1:8,  ^3:GRV, BSPC,        INS, HOME,PGUP,    NLCK,   PSLS:^9, PAST:^0,   PMNS:^4,
    TAB,      SCLN,  COMM,   DOT,     P,       Y,    F,     G,    C,    R,      L,      SLSH,  ^2:^6,  BSLS,        DEL, END, PGDN,    P1:^A,  P2:^B,   P3:^C,
    CAPS,     A,     O,      E,       U,       I,    D,     H,    T,    N,      S,      MINS,          ENT,                            P4:^D,  P5:^E,   P6:^F,     PPLS:COMM,
    LSFT,     QUOT,  Q,      J,       K,       X,    B,     M,    W,    V,      Z,                     RSFT,             UP,           P7:EQL, P8:X,    P9:^SCLN,
    LCTL,     LGUI,  LALT,                     SPC,                             RALT,   RGUI,  APP,    RCTL,        LEFT,DOWN,RGHT,    P0:BSLS,         PDOT:SCLN, PENT
    """,
    ]
