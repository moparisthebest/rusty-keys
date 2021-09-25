
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::hash::Hash;
use std::convert::TryFrom;

use crate::{Error, Result};

const INVERT_KEY_FLAG: char = '^';
const CAPS_MODIFY_KEY_FLAG: char = '*';
const HALF_KEY_SEPARATOR: char = ':';

// nightly only...
//pub trait KeyCode = Into<usize> + TryFrom<usize> + Copy + Clone + Eq + Hash + Default + 'static;

#[derive(PartialEq)]
pub enum KeyState {
    DOWN,
    UP,
    OTHER,
}

pub trait KeyEvent<T>
    where
        T: Into<usize>,
{
    fn code(&self) -> T;
    fn value(&self) -> KeyState;
}

pub trait Keyboard<T, E, R = ()>
    where
        T: Into<usize>,
        E: KeyEvent<T>,
{
    fn send(&self, event: &mut E) -> Result<R>;
    fn send_mod_code(&self, code: T, event: &mut E) -> Result<R>;
    fn send_mod_code_value(&self, code: T, up_not_down: bool, event: &mut E) -> Result<R>;
    fn synchronize(&self) -> Result<R>;
    fn left_shift_code(&self) -> T;
    fn right_shift_code(&self) -> T;
    fn caps_lock_code(&self) -> T;
    fn block_key(&self) -> Result<R>;
}

pub trait KeyMapper<K, T, E, R>
    where
        T: Into<usize>,
        E: KeyEvent<T>,
        K: Keyboard<T, E, R>,
{
    fn send_event(&self, key_state: &[bool], event: &mut E, device: &K) -> Result<R>;
}

pub struct KeyMaps<K, T, E, R = ()>
    where
        T: Into<usize> + Copy + Clone + Eq + Hash,
        E: KeyEvent<T>,
        K: Keyboard<T, E, R>,
{
    keymaps: Vec<Box<dyn KeyMapper<K, T, E, R>>>,
    keymap_index_keys: HashMap<T, usize>,
    switch_layout_keys: Vec<usize>,
    key_state: [bool; KEY_MAX],
    revert_default_keys: Vec<T>,
    revert_keymap_index: usize,
    // above do not change, below does
    chosen_keymap_index: usize,
    current_keymap_index: usize,
}

fn parse_key<T: Clone + Copy>(key_map: &HashMap<&'static str, T>, key: &str) -> T {
    match key_map.get(key.trim_matches(|c: char| c.is_whitespace() || c == INVERT_KEY_FLAG || c == CAPS_MODIFY_KEY_FLAG)) {
        Some(key_code) => *key_code,
        None => panic!("unknown key: {}", key.trim())
    }
}

fn parse_keymap_numeric<T: Clone + Copy>(key_map: &HashMap<&'static str, T>, keymap: &str) -> Vec<T> {
    keymap.split(",").map(|k| parse_key(key_map, k)).collect()
}

fn parse_key_half_inverted<T: Clone + Copy>(key_map: &HashMap<&'static str, T>, key: &str) -> HalfInvertedKey<T> {
    HalfInvertedKey {
        code: parse_key(key_map, key),
        invert_shift: key.contains(INVERT_KEY_FLAG),
        capslock_nomodify: key.contains(CAPS_MODIFY_KEY_FLAG),
    }
}

// maybe shortcut to this if not contains * or :
fn parse_keymap_u16<T: Clone + Copy>(key_map: &HashMap<&'static str, T>, keymap: &str) -> Vec<T> {
    keymap.split(",").map(|k| parse_key(key_map, k)).collect()
}

// todo: how do I return an iterator here instead of .collect to Vec?
fn parse_keymap<T: Copy>(key_map: &HashMap<&'static str, T>, keymap: &str) -> Vec<Key<T>> {
    keymap.split(",").map(|k| {
        let ret: Key<T> = if k.contains(HALF_KEY_SEPARATOR) {
            let keys: Vec<&str> = k.split(HALF_KEY_SEPARATOR).collect();
            if keys.len() != 2 {
                panic!("split key can only have 2 keys, 1 :, has {} keys", keys.len());
            }
            let mut shift_half = parse_key_half_inverted(key_map, keys[1]);
            shift_half.invert_shift = !shift_half.invert_shift;
            Key::FullKey(parse_key_half_inverted(key_map, keys[0]), shift_half)
        } else if k.contains(INVERT_KEY_FLAG) || k.contains(CAPS_MODIFY_KEY_FLAG) {
            Key::HalfKey(parse_key_half_inverted(key_map, k))
        } else {
            Key::Direct(parse_key(key_map, k))
        };
        ret
    }).collect()
}

impl<K, T, E, R> KeyMaps<K, T, E, R>
    where
        T: Into<usize> + TryFrom<usize> + Copy + Clone + Eq + Hash + Default + 'static,
        E: KeyEvent<T>,
        K: Keyboard<T, E, R>,
{
    #[cfg(feature = "toml_serde")]
    pub fn from_cfg<P: AsRef<Path>>(key_map: &HashMap<&'static str, T>, path: P) -> KeyMaps<K, T, E, R> {
        let key_map_config = parse_cfg(path).expect("provided config cannot be found/parsed");
        KeyMaps::new(key_map, key_map_config)
    }

    pub fn new(key_map: &HashMap<&'static str, T>, config: KeymapConfig) -> KeyMaps<K, T, E, R> {
        if config.keymaps.len() < 2 {
            panic!("must have at least 2 keymaps (original and mapped) but only have {},", config.keymaps.len());
        }
        if config.default_keymap_index >= config.keymaps.len() || config.revert_keymap_index >= config.keymaps.len() {
            panic!("default_keymap_index ({}) and revert_keymap_index ({}) must be less than keymaps length ({}),", config.default_keymap_index, config.revert_keymap_index, config.keymaps.len());
        }
        let base_keymap = parse_keymap_numeric(key_map, &config.keymaps[0]);
        //println!("base_keymap      : {:?}", base_keymap);
        let mut keymaps: Vec<Box<dyn KeyMapper<K, T, E, R>>> = vec!(Box::new(Key::Noop)); // todo: can we share the box?
        let mut keymap_index_keys: HashMap<T, usize> = HashMap::new();
        for (x, v) in config.keymaps.iter().enumerate() {
            keymap_index_keys.insert(*key_map.get(&*x.to_string()).unwrap(), x);
            if x == 0 {
                continue;
            }
            if v.contains(HALF_KEY_SEPARATOR) || v.contains(INVERT_KEY_FLAG) || v.contains(CAPS_MODIFY_KEY_FLAG) {
                // we need KeyMap, the complicated more memory taking one
                let v = parse_keymap(key_map, v);
                let mut keymap = KeyMap::new();
                let mut i: usize = 0;
                for key_code in v {
                    // todo: if these are the same, do Noop instead
                    keymap.map(base_keymap[i], key_code);
                    i = i + 1;
                    if i > base_keymap.len() {
                        panic!("all keymaps must be the same length, keymap index 0 length: {}, index {} length: {},", base_keymap.len(), x, i);
                    }
                }
                keymaps.push(Box::new(keymap));
            } else {
                // this is a simple keymap
                let v = parse_keymap_u16(key_map, v);
                let mut keymap = CodeKeyMap::new();
                let mut i: usize = 0;
                for key_code in v {
                    keymap.map(base_keymap[i], key_code);
                    i = i + 1;
                    if i > base_keymap.len() {
                        panic!("all keymaps must be the same length, keymap index 0 length: {}, index {} length: {},", base_keymap.len(), x, i);
                    }
                }
                keymaps.push(Box::new(keymap));
            }
        }
        //println!("keymaps: {:?}", keymaps);
        //println!("keymap_index_keys: {:?}", keymap_index_keys);

        let mut revert_default_keys = Vec::new();
        if config.revert_default_key.is_some() {
            revert_default_keys.push(parse_key(key_map, &config.revert_default_key.unwrap()));
        }
        if config.revert_default_keys.is_some() {
            for revert_default_key in config.revert_default_keys.unwrap() {
                let revert_default_key = parse_key(key_map, &revert_default_key);
                if !revert_default_keys.contains(&revert_default_key) {
                    revert_default_keys.push(revert_default_key);
                }
            }
        }
        // revert_default_keys may be empty, but that's ok

        KeyMaps {
            keymaps: keymaps,
            keymap_index_keys: keymap_index_keys,
            switch_layout_keys: config.switch_layout_keys.iter().map(|k| parse_key(key_map, k).into()).collect(),
            key_state: [false; KEY_MAX],
            // todo: detect key state? at least CAPSLOCK...
            revert_default_keys,
            revert_keymap_index: config.revert_keymap_index,
            chosen_keymap_index: config.default_keymap_index,
            current_keymap_index: config.default_keymap_index,
        }
    }
//}

//impl KeyMapper for KeyMaps {
//impl KeyMaps {
pub fn send_event(&mut self, mut event: &mut E, device: &K) -> Result<R> {
        //println!("type: {} code: {} value: {}", event.type_, event.code, event.value);
    let value = event.value();
    if value != KeyState::OTHER {
            // todo: index check here...
        if event.code() == device.caps_lock_code() {
            if value == KeyState::DOWN {
                self.key_state[device.caps_lock_code().into()] = !self.key_state[device.caps_lock_code().into()];
                }
            } else {
            self.key_state[event.code().into()] = value == KeyState::DOWN;
            }
            let mut switch_layout_keys_pressed = true;
            for layout_switch_key in self.switch_layout_keys.iter_mut() {
                if !self.key_state[*layout_switch_key] {
                    switch_layout_keys_pressed = false;
                    break;
                }
            }
            //println!("switch_layout_keys_pressed: {}", self.switch_layout_keys_pressed);
            if switch_layout_keys_pressed {
                let new_index = self.keymap_index_keys.get(&event.code());
                if new_index.is_some() {
                    self.chosen_keymap_index = *new_index.unwrap();
                    self.current_keymap_index = self.chosen_keymap_index; // todo: what if revert_default_key is held? for now ignore
                    return device.block_key(); // we don't want to also send this keypress, so bail
                }
            }
        if self.revert_default_keys.contains(&event.code()) {
            match value {
                    // todo: ctrl+c will get c stuck because code c value 1 will be sent, but then we'll let go of ctrl, and code j value 0 is sent, so c is never released... fix that...
                KeyState::DOWN => self.current_keymap_index = self.revert_keymap_index,
                KeyState::UP => self.current_keymap_index = self.chosen_keymap_index,
                    _ => () // do nothing for 2
                }
            }
        }
        self.keymaps[self.current_keymap_index].send_event(&self.key_state, &mut event, device)
    }
}

// 249 is one more than KEY_MICMUTE which is max key in uinput-sys event.rs
const KEY_MAX: usize = 249;

struct KeyMap<T: Into<usize> + Copy> {
    //keymap: Vec<Key>,
    keymap: [Key<T>; KEY_MAX],
}

impl<T: Into<usize> + Copy> KeyMap<T> {
    pub fn new() -> Self {
        //let mut keymap = [0u16; KEY_MAX];
        //let mut keymap : [Box<KeyMapper>; KEY_MAX] = [Box::new(NOOP); KEY_MAX];
        //let mut keymap : [Box<KeyMapper>; KEY_MAX] = [Box::new(0u16); KEY_MAX];
        let keymap: [Key<T>; KEY_MAX] = [Key::Noop; KEY_MAX];
        /*
        let mut keymap: Vec<Key> = Vec::with_capacity(KEY_MAX);
        #[allow(unused_variables)]
        for x in 0..KEY_MAX {
            keymap.push(Key::Noop);
        }
        */
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
    pub fn map(&mut self, from: T, to: Key<T>) {
        self.keymap[from.into()] = to;
    }
}

impl<K, T, E, R> KeyMapper<K, T, E, R> for KeyMap<T>
    where
        T: Into<usize> + Copy,
        E: KeyEvent<T>,
        K: Keyboard<T, E, R>,
{
    fn send_event(&self, key_state: &[bool], event: &mut E, device: &K) -> Result<R> {
        self.keymap[event.code().into()].send_event(key_state, event, device)
    }
}

struct CodeKeyMap<T: Into<usize> + TryFrom<usize> + Copy + Default> {
    //keymap: Vec<Key>,
    keymap: [T; KEY_MAX],
}

impl<T: Into<usize> + TryFrom<usize> + Copy + Default> CodeKeyMap<T> {
    pub fn new() -> Self {
        let mut keymap = [T::default(); KEY_MAX];
        // which is rustier
        /*
        for x  in 0..KEY_MAX {
            keymap[x as usize] = x as u16;
        }
        */
        for (x, v) in keymap.iter_mut().enumerate() {
            *v = T::try_from(x).unwrap_or_else(|_| panic!("cannot convert from usize to T ????"));
        }
        //println!("keymap: {:?}", &keymap[..]);
        CodeKeyMap {
            keymap: keymap
        }
    }

    pub fn map(&mut self, from: T, to: T) {
        self.keymap[from.into()] = to;
    }
}

impl<K, T, E, R> KeyMapper<K, T, E, R> for CodeKeyMap<T>
    where
        T: Into<usize> + TryFrom<usize> + Copy + Default,
        E: KeyEvent<T>,
        K: Keyboard<T, E, R>,
{
    fn send_event(&self, _key_state: &[bool], event: &mut E, device: &K) -> Result<R> {
        device.send_mod_code(self.keymap[event.code().into()], event)
        //self.keymap[event.code().into()].send_event(key_state, event, device)
    }
}

// todo:capslock_nomodify is like a whole-key thing, not a half-key thing, split code/invert_shift to own struct, send into send_key from *InvertedKey, maybe anyway, consider it, maybe 1 char for whole key and another for half?
#[derive(Clone, Copy)]
struct HalfInvertedKey<T: Clone + Copy> {
    code: T,
    // code this is describing
    invert_shift: bool,
    // true to invert shift for this code
    capslock_nomodify: bool,
    // true means capslock does not normally modify this, but you would like it to
}

fn send_half_inverted_key<K, T, E, R>(half_inverted_key: &HalfInvertedKey<T>, event: &mut E, device: &K, left_shift: bool, right_shift: bool, caps_lock: bool) -> Result<R>
    where
        T: Into<usize> + Clone + Copy,
        E: KeyEvent<T>,
        K: Keyboard<T, E, R>,
{
    let value = event.value();
    let mut invert_shift = half_inverted_key.invert_shift;
    if value == KeyState::DOWN {
        if caps_lock && half_inverted_key.capslock_nomodify {
            invert_shift = !invert_shift;
        }
        if invert_shift {
            let (shift_code, up_not_down) = if left_shift {
                (device.left_shift_code(), true)
            } else if right_shift {
                (device.right_shift_code(), true)
            } else {
                (device.left_shift_code(), false)
            };
            device.send_mod_code_value(shift_code, up_not_down, event)?;
            // SYN_REPORT after, then key, then key's SYN_REPORT
            device.synchronize()?;
        }
    }
    let ret = device.send_mod_code(half_inverted_key.code, event)?;
    if value == KeyState::UP {
        if caps_lock && half_inverted_key.capslock_nomodify {
            invert_shift = !invert_shift;
        }
        if invert_shift {
            let (shift_code, up_not_down) = if left_shift {
                (device.left_shift_code(), false)
            } else if right_shift {
                (device.right_shift_code(), false)
            } else {
                (device.left_shift_code(), true)
            };
            // SYN_REPORT first after key, then shift, then key's SYN_REPORT which will be used for shift's
            device.synchronize()?;
            device.send_mod_code_value(shift_code, up_not_down, event)?;
        }
    }
    Ok(ret)
}

impl<K, T, E, R> KeyMapper<K, T, E, R> for HalfInvertedKey<T>
    where
        T: Into<usize> + Clone + Copy,
        E: KeyEvent<T>,
        K: Keyboard<T, E, R>,
{
    fn send_event(&self, key_state: &[bool], event: &mut E, device: &K) -> Result<R> {
        let left_shift = key_state[device.left_shift_code().into()];
        let right_shift = key_state[device.right_shift_code().into()];
        let caps_lock = key_state[device.caps_lock_code().into()];
        send_half_inverted_key(self, event, device, left_shift, right_shift, caps_lock)
    }
}

#[derive(Clone, Copy)]
enum Key<T>
    where
        T: Copy + Clone
{
    Noop,
    Direct(T),
    HalfKey(HalfInvertedKey<T>),
    FullKey(HalfInvertedKey<T>, HalfInvertedKey<T>),
}

impl<K, T, E, R> KeyMapper<K, T, E, R> for Key<T>
    where
        T: Into<usize> + Copy,
        E: KeyEvent<T>,
        K: Keyboard<T, E, R>,
{
    fn send_event(&self, key_state: &[bool], event: &mut E, device: &K) -> Result<R> {
        match *self {
            Key::Noop => {
                device.send(event)
            },
            Key::Direct(code) => {
                device.send_mod_code(code, event)
            },
            Key::HalfKey(ref key_half) => {
                key_half.send_event(key_state, event, device)
            },
            Key::FullKey(ref noshift_half, ref shift_half) => {
                let left_shift = key_state[device.left_shift_code().into()];
                let right_shift = key_state[device.right_shift_code().into()];
                let caps_lock = key_state[device.caps_lock_code().into()];
                if caps_lock != (left_shift || right_shift) {
                    send_half_inverted_key(shift_half, event, device, left_shift, right_shift, caps_lock)
                } else {
                    send_half_inverted_key(noshift_half, event, device, left_shift, right_shift, caps_lock)
                }
            },
        }
    }
}

use std::path::Path;

#[cfg(feature = "toml_serde")]
#[derive(serde::Deserialize, Debug)]
pub struct KeymapConfig {
    switch_layout_keys: Vec<String>,
    revert_default_key: Option<String>,
    revert_default_keys: Option<Vec<String>>,
    revert_keymap_index: usize,
    default_keymap_index: usize,
    keymaps: Vec<String>
}

#[cfg(feature = "toml_serde")]
fn parse_cfg<P: AsRef<Path>>(path: P) -> Result<KeymapConfig> {
    let mut f = File::open(path)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    toml::from_str(&input).map_err(|e| Error::Toml(e))
}

#[cfg(not(feature = "toml_serde"))]
#[derive(Debug)]
pub struct KeymapConfig {
    switch_layout_keys: Vec<&'static str>,
    revert_default_key: Option<&'static str>,
    revert_default_keys: Option<Vec<&'static str>>,
    revert_keymap_index: usize,
    default_keymap_index: usize,
    keymaps: Vec<&'static str>
}

#[cfg(not(feature = "toml_serde"))]
impl Default for KeymapConfig {
    fn default() -> Self {
        KeymapConfig {
            switch_layout_keys: vec!["LEFTSHIFT", "RIGHTSHIFT"],

            // pressing any of these keys reverts to the index specified in revert_keymap_index for only the duration of the pressing
            // used so QWERTY shortcuts like Ctrl+C still work
            revert_default_keys: Some(vec!["LCTL", "LGUI", "LALT"]),
            revert_keymap_index: 0,

            // this is the default index to use when the program first starts
            // in this case, 2 means Unix Programmer Dvorak
            default_keymap_index: 2,

            // these are the keymaps available, you can add as many as you want or re-order them, just be aware the mapping is
            // always done from the first one to all subsequent ones, so you probably want to leave QWERTY or similar up top
            keymaps: vec![
                // default key layout, QWERTY in this case
                r###"
                ESC, F1,  F2,  F3,  F4,  F5,  F6,  F7,  F8,  F9,  F10, F11, F12,           PSCR,SLCK,BRK,
                GRV, 1,   2,   3,   4,   5,   6,   7,   8,   9,   0,   MINS,EQL, BSPC,     INS, HOME,PGUP,    NLCK,PSLS,PAST,PMNS,
                TAB, Q,   W,   E,   R,   T,   Y,   U,   I,   O,   P,   LBRC,RBRC,BSLS,     DEL, END, PGDN,    P7,  P8,  P9,
                CAPS,A,   S,   D,   F,   G,   H,   J,   K,   L,   SCLN,QUOT,     ENT,                         P4,  P5,  P6,  PPLS,
                LSFT,Z,   X,   C,   V,   B,   N,   M,   COMM,DOT, SLSH,          RSFT,          UP,           P1,  P2,  P3,
                LCTL,LGUI,LALT,          SPC,                     RALT,RGUI,APP, RCTL,     LEFT,DOWN,RGHT,    P0,       PDOT,PENT
                "###,
                // Dvorak http://en.wikipedia.org/wiki/Dvorak_Simplified_Keyboard
                // https://www.moparisthebest.com/kbs/standard-dvorak-QwertySecondary.svg
                r###"
                ESC, F1,  F2,  F3,  F4,  F5,  F6,  F7,  F8,  F9,  F10, F11, F12,           PSCR,SLCK,BRK,
                GRV, 1,   2,   3,   4,   5,   6,   7,   8,   9,   0,   LBRC,RBRC,BSPC,     INS, HOME,PGUP,    NLCK,PSLS,PAST,PMNS,
                TAB, QUOT,COMM,DOT, P,   Y,   F,   G,   C,   R,   L,   SLSH,EQL, BSLS,     DEL, END, PGDN,    P7,  P8,  P9,
                CAPS,A,   O,   E,   U,   I,   D,   H,   T,   N,   S,   MINS,     ENT,                         P4,  P5,  P6,  PPLS,
                LSFT,SCLN,Q,   J,   K,   X,   B,   M,   W,   V,   Z,             RSFT,          UP,           P1,  P2,  P3,
                LCTL,LGUI,LALT,          SPC,                     RALT,RGUI,APP, RCTL,     LEFT,DOWN,RGHT,    P0,       PDOT,PENT
                "###,
                // Unix Programmer Dvorak - for unix developers who are switching from dvorak
                // https://www.moparisthebest.com/kbs/programmer-dvorak-NoSecondary-NumpadStandard-NoSwap-StandardNums-SwapAt-SwapPipe.svg
                r###"
                ESC,      F1,    F2,     F3,      F4,      F5,   F6,    F7,   F8,   F9,     F10,    F11,   F12,                 PSCR,SLCK,BRK,
                *^4:*^GRV,  *^7:*1,  *LBRC:*2, *^LBRC:*3, *^RBRC:*4, *^9:*5, *^2:*6,  *^8:*7, *^0:*8, *^EQL:*9, *RBRC:*0, *^1:*^5, *^3:*GRV, BSPC,        INS, HOME,PGUP,    NLCK,   *PSLS:*^9, *PAST:*^0,   *PMNS:*^4,
                TAB,      *QUOT,  *COMM,   *DOT,     P,       Y,    F,     G,    C,    R,      L,      *SLSH,  *EQL:*^6, *^BSLS,  DEL, END, PGDN,    *P7:^A,  *P8:^B,   *P9:^C,
                CAPS,     A,     O,      E,       U,       I,    D,     H,    T,    N,      S,      *MINS,          ENT,                            *P4:^D,  *P5:^E,   *P6:^F,     *PPLS:*COMM,
                LSFT,     *SCLN,  Q,      J,       K,       X,    B,     M,    W,    V,      Z,                     RSFT,             UP,           *P1:*EQL, *P2:X,    *P3:*^SCLN,
                LCTL,     LGUI,  LALT,                     SPC,                             RALT,   RGUI,  APP,    RCTL,        LEFT,DOWN,RGHT,    *P0:*BSLS,         *PDOT:*SCLN, PENT
                "###,
            ],
            revert_default_key: None, // use revert_default_keys instead
        }
    }
}

