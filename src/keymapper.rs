use std::{collections::HashMap, convert::TryFrom, hash::Hash};

#[cfg(feature = "toml_serde")]
use std::path::Path;

use crate::Result;

const INVERT_KEY_FLAG: char = '^';
const CAPS_MODIFY_KEY_FLAG: char = '*';
const HALF_KEY_SEPARATOR: char = ':';

// nightly only...
//pub trait KeyCode = Into<usize> + TryFrom<usize> + Copy + Clone + Eq + Hash + Default + 'static;

#[derive(PartialEq, Debug)]
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
    T: Into<usize> + Copy,
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

    fn send_half_inverted_key(
        &self,
        half_inverted_key: &HalfInvertedKey<T>,
        event: &mut E,
        left_shift: bool,
        right_shift: bool,
        caps_lock: bool,
    ) -> Result<R> {
        let value = event.value();
        let mut invert_shift = half_inverted_key.invert_shift;
        if value == KeyState::DOWN {
            if caps_lock && half_inverted_key.capslock_nomodify {
                invert_shift = !invert_shift;
            }
            if invert_shift {
                let (shift_code, up_not_down) = if left_shift {
                    (self.left_shift_code(), true)
                } else if right_shift {
                    (self.right_shift_code(), true)
                } else {
                    (self.left_shift_code(), false)
                };
                self.send_mod_code_value(shift_code, up_not_down, event)?;
                // SYN_REPORT after, then key, then key's SYN_REPORT
                self.synchronize()?;
            }
        }
        let ret = self.send_mod_code(half_inverted_key.code, event)?;
        if value == KeyState::UP {
            if caps_lock && half_inverted_key.capslock_nomodify {
                invert_shift = !invert_shift;
            }
            if invert_shift {
                let (shift_code, up_not_down) = if left_shift {
                    (self.left_shift_code(), false)
                } else if right_shift {
                    (self.right_shift_code(), false)
                } else {
                    (self.left_shift_code(), true)
                };
                // SYN_REPORT first after key, then shift, then key's SYN_REPORT which will be used for shift's
                self.synchronize()?;
                self.send_mod_code_value(shift_code, up_not_down, event)?;
            }
        }
        Ok(ret)
    }
}

pub trait KeyMapper<K, T, E, R>
where
    T: Into<usize> + Copy,
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
    pub devices: DeviceMatchers,
    // above do not change, below does
    chosen_keymap_index: usize,
    current_keymap_index: usize,
}

fn parse_key<T: Clone + Copy>(key_map: &HashMap<&'static str, T>, key: &str) -> T {
    match key_map.get(key.trim_matches(|c: char| {
        c.is_whitespace() || c == INVERT_KEY_FLAG || c == CAPS_MODIFY_KEY_FLAG
    })) {
        Some(key_code) => *key_code,
        None => panic!("unknown key: {}", key.trim()),
    }
}

fn parse_keymap_numeric<T: Clone + Copy>(
    key_map: &HashMap<&'static str, T>,
    keymap: &str,
) -> Vec<T> {
    keymap.split(",").map(|k| parse_key(key_map, k)).collect()
}

fn parse_key_half_inverted<T: Clone + Copy>(
    key_map: &HashMap<&'static str, T>,
    key: &str,
) -> HalfInvertedKey<T> {
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
    keymap
        .split(",")
        .map(|k| {
            let ret: Key<T> = if k.contains(HALF_KEY_SEPARATOR) {
                let keys: Vec<&str> = k.split(HALF_KEY_SEPARATOR).collect();
                if keys.len() != 2 {
                    panic!(
                        "split key can only have 2 keys, 1 :, has {} keys",
                        keys.len()
                    );
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
        })
        .collect()
}

impl<K, T, E, R> KeyMaps<K, T, E, R>
where
    T: Into<usize> + TryFrom<usize> + Copy + Clone + Eq + Hash + Default + 'static,
    E: KeyEvent<T>,
    K: Keyboard<T, E, R>,
{
    #[cfg(feature = "toml_serde")]
    pub fn from_cfg<P: AsRef<Path>>(
        key_map: &HashMap<&'static str, T>,
        path: P,
    ) -> KeyMaps<K, T, E, R> {
        let key_map_config = parse_cfg(path).expect("provided config cannot be found/parsed");
        KeyMaps::new(key_map, key_map_config)
    }

    pub fn new(key_map: &HashMap<&'static str, T>, config: KeymapConfig) -> KeyMaps<K, T, E, R> {
        if config.keymaps.len() < 2 {
            panic!(
                "must have at least 2 keymaps (original and mapped) but only have {},",
                config.keymaps.len()
            );
        }
        if config.default_keymap_index >= config.keymaps.len()
            || config.revert_keymap_index >= config.keymaps.len()
        {
            panic!("default_keymap_index ({}) and revert_keymap_index ({}) must be less than keymaps length ({}),", config.default_keymap_index, config.revert_keymap_index, config.keymaps.len());
        }
        let base_keymap = parse_keymap_numeric(key_map, &config.keymaps[0]);
        //println!("base_keymap      : {:?}", base_keymap);
        let mut keymaps: Vec<Box<dyn KeyMapper<K, T, E, R>>> = vec![Box::new(Key::Noop)]; // todo: can we share the box?
        let mut keymap_index_keys: HashMap<T, usize> = HashMap::new();
        for (x, v) in config.keymaps.iter().enumerate() {
            keymap_index_keys.insert(*key_map.get(&*x.to_string()).unwrap(), x);
            if x == 0 {
                continue;
            }
            if v.contains(HALF_KEY_SEPARATOR)
                || v.contains(INVERT_KEY_FLAG)
                || v.contains(CAPS_MODIFY_KEY_FLAG)
            {
                // we need KeyMap, the complicated more memory taking one
                let v = parse_keymap(key_map, v);
                let mut keymap = KeyMap::new();
                let mut i: usize = 0;
                for key_code in v {
                    // if it's a direct key and it's the same, don't do any mapping
                    if let Key::Direct(key) = key_code {
                        if base_keymap[i] != key {
                            keymap.map(base_keymap[i], key_code);
                        }
                    } else {
                        keymap.map(base_keymap[i], key_code);
                    }
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
                    if base_keymap[i] != key_code {
                        keymap.map(base_keymap[i], key_code);
                    }
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
            switch_layout_keys: config
                .switch_layout_keys
                .iter()
                .map(|k| parse_key(key_map, k).into())
                .collect(),
            key_state: [false; KEY_MAX],
            // todo: detect key state? at least CAPSLOCK...
            revert_default_keys,
            revert_keymap_index: config.revert_keymap_index,
            chosen_keymap_index: config.default_keymap_index,
            current_keymap_index: config.default_keymap_index,
            devices: config.devices,
        }
    }
    //}

    //impl KeyMapper for KeyMaps {
    //impl KeyMaps {
    pub fn send_event(&mut self, mut event: &mut E, device: &K) -> Result<R> {
        let value = event.value();
        if value != KeyState::OTHER {
            // todo: index check here...
            if event.code() == device.caps_lock_code() {
                if value == KeyState::DOWN {
                    self.key_state[device.caps_lock_code().into()] =
                        !self.key_state[device.caps_lock_code().into()];
                }
            } else {
                let idx = event.code().into();
                if idx >= KEY_MAX {
                    // oh well, send it directly then
                    return device.send(event);
                }
                self.key_state[idx] = value == KeyState::DOWN;
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
                    KeyState::DOWN => {
                        // todo: should we release currently held keys and then press them back down here, kinda the opposite of below? not for now...
                        self.current_keymap_index = self.revert_keymap_index
                    }
                    KeyState::UP => {
                        self.current_keymap_index = self.chosen_keymap_index;
                        #[cfg(not(target_os = "macos"))]
                        {
                            // need to release all currently held down keys, except this one, otherwise ctrl+c will get c stuck because code c value 1 will be sent, but then we'll let go of ctrl, and code j value 0 is sent, so c is never released
                            let orig_code = event.code();
                            for (idx, key_down) in self.key_state.iter_mut().enumerate() {
                                if *key_down {
                                    device.send_mod_code_value(
                                        T::try_from(idx).unwrap_or_else(|_| {
                                            panic!("cannot convert from usize to T ????")
                                        }),
                                        true,
                                        event,
                                    )?;
                                    *key_down = false;
                                }
                            }
                            // todo: seems like we should not send this here, and instead just set the original code back, and pass it through the keymaps?
                            return device.send_mod_code_value(orig_code, true, event);
                        }
                    }
                    _ => (), // do nothing for 2
                }
            }
        }
        self.keymaps[self.current_keymap_index].send_event(&self.key_state, &mut event, device)
    }
}

// 249 is one more than KEY_MICMUTE which is max key in uinput-sys event.rs
const KEY_MAX: usize = 249;

struct KeyMap<T: Into<usize> + Copy> {
    keymap: [Key<T>; KEY_MAX],
}

impl<T: Into<usize> + Copy> KeyMap<T> {
    pub fn new() -> Self {
        KeyMap {
            keymap: [Key::Noop; KEY_MAX],
        }
    }

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
    keymap: [T; KEY_MAX],
}

impl<T: Into<usize> + TryFrom<usize> + Copy + Default> CodeKeyMap<T> {
    pub fn new() -> Self {
        let mut keymap = [T::default(); KEY_MAX];
        for (x, v) in keymap.iter_mut().enumerate() {
            *v = T::try_from(x).unwrap_or_else(|_| panic!("cannot convert from usize to T ????"));
        }
        //println!("keymap: {:?}", &keymap[..]);
        CodeKeyMap { keymap }
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
pub struct HalfInvertedKey<T: Clone + Copy> {
    pub code: T,
    // code this is describing
    pub invert_shift: bool,
    // true to invert shift for this code
    pub capslock_nomodify: bool,
    // true means capslock does not normally modify this, but you would like it to
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
        device.send_half_inverted_key(self, event, left_shift, right_shift, caps_lock)
    }
}

#[derive(Clone, Copy)]
enum Key<T>
where
    T: Copy + Clone,
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
            Key::Noop => device.send(event),
            Key::Direct(code) => device.send_mod_code(code, event),
            Key::HalfKey(ref key_half) => key_half.send_event(key_state, event, device),
            Key::FullKey(ref noshift_half, ref shift_half) => {
                let left_shift = key_state[device.left_shift_code().into()];
                let right_shift = key_state[device.right_shift_code().into()];
                let caps_lock = key_state[device.caps_lock_code().into()];
                if caps_lock != (left_shift || right_shift) {
                    device.send_half_inverted_key(
                        shift_half,
                        event,
                        left_shift,
                        right_shift,
                        caps_lock,
                    )
                } else {
                    device.send_half_inverted_key(
                        noshift_half,
                        event,
                        left_shift,
                        right_shift,
                        caps_lock,
                    )
                }
            }
        }
    }
}

pub trait DeviceIds {
    fn bustype(&self) -> Option<u16>;
    fn vendor(&self) -> Option<u16>;
    fn product(&self) -> Option<u16>;
    fn version(&self) -> Option<u16>;
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "toml_serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "toml_serde", serde(deny_unknown_fields))]
pub struct DeviceMatcher {
    pub bustype: Option<u16>,
    pub vendor: Option<u16>,
    pub product: Option<u16>,
    pub version: Option<u16>,
}

#[derive(Debug)]
#[cfg_attr(feature = "toml_serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "toml_serde", serde(deny_unknown_fields))]
pub struct DeviceMatchers {
    #[cfg_attr(feature = "toml_serde", serde(default))]
    grab: Vec<DeviceMatcher>,
    #[cfg_attr(feature = "toml_serde", serde(default = "default_skip"))]
    skip: Vec<DeviceMatcher>,
}

impl Default for DeviceMatchers {
    fn default() -> Self {
        Self {
            grab: Default::default(),
            skip: default_skip(),
        }
    }
}

pub fn default_skip() -> Vec<DeviceMatcher> {
    vec![
        // 0x1050 is Yubico
        DeviceMatcher {
            vendor: Some(0x1050),
            ..Default::default()
        },
    ]
}

#[inline]
fn um(rule: Option<u16>, data: Option<u16>) -> bool {
    rule.is_none() || rule == data
}

impl DeviceMatcher {
    pub fn matches(&self, device: &dyn DeviceIds) -> bool {
        um(self.bustype, device.bustype())
            && um(self.vendor, device.vendor())
            && um(self.product, device.product())
            && um(self.version, device.version())
    }
}

impl DeviceMatchers {
    pub fn grab(&self, device: &dyn DeviceIds) -> bool {
        // we should grab this device if grab is empty or any single entry matches
        (self.grab.is_empty() || self.grab.iter().any(|d| d.matches(device)))
            // and skip is empty or no skip entry matches
            && (self.skip.is_empty() || !self.skip.iter().any(|d| d.matches(device)))
    }
}

#[cfg(feature = "toml_serde")]
#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct KeymapConfig {
    switch_layout_keys: Vec<String>,
    revert_default_key: Option<String>,
    revert_default_keys: Option<Vec<String>>,
    revert_keymap_index: usize,
    default_keymap_index: usize,
    keymaps: Vec<String>,
    #[serde(default)]
    devices: DeviceMatchers,
}

#[cfg(feature = "toml_serde")]
fn parse_cfg<P: AsRef<Path>>(path: P) -> Result<KeymapConfig> {
    use std::io::Read;
    let mut f = std::fs::File::open(path)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    toml::from_str(&input).map_err(|e| crate::Error::Toml(e))
}

#[cfg(not(feature = "toml_serde"))]
#[derive(Debug)]
pub struct KeymapConfig {
    switch_layout_keys: Vec<&'static str>,
    revert_default_key: Option<&'static str>,
    revert_default_keys: Option<Vec<&'static str>>,
    revert_keymap_index: usize,
    default_keymap_index: usize,
    keymaps: Vec<&'static str>,
    devices: DeviceMatchers,
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
            devices: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use libc::input_id;

    use super::*;

    #[test]
    fn test_device_matchers() {
        // test default
        let devices: DeviceMatchers = toml::from_str("").unwrap();
        assert!(devices.grab.is_empty());
        assert_eq!(devices.skip.len(), 1);
        let yubi = input_id {
            vendor: 0x1050,
            product: 0x0406,
            bustype: 0,
            version: 0,
        };
        let yubi2 = input_id {
            vendor: 0x1050,
            product: 0x0407,
            bustype: 0,
            version: 0,
        };
        let ergosnm = input_id {
            vendor: 0x5a69,
            product: 0xe200,
            bustype: 0,
            version: 0,
        };
        let rando = input_id {
            vendor: 1,
            product: 2,
            bustype: 3,
            version: 4,
        };
        assert!(!devices.grab(&yubi));
        assert!(!devices.grab(&yubi2));
        assert!(devices.grab(&ergosnm));
        assert!(devices.grab(&rando));
        let devices: DeviceMatchers = toml::from_str(
            r###"
            # yubikey
            [[skip]]
            vendor = 0x1050
            # ergosnm
            [[skip]]
            vendor = 0x5a69
            product = 0xe200
            "###,
        )
        .unwrap();
        assert!(!devices.grab(&yubi));
        assert!(!devices.grab(&yubi2));
        assert!(!devices.grab(&ergosnm));
        assert!(devices.grab(&rando));
    }
}
