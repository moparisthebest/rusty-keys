
use crate::Device;
use libc::{c_int, input_event};
use uinput_sys::{KEY_LEFTSHIFT, KEY_RIGHTSHIFT, KEY_CAPSLOCK};

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use crate::{Error, Result};

// 1 is down, 0 is up
const DOWN: i32 = 1;
const UP: i32 = 0;
const INVERT_KEY_FLAG: char = '^';
const CAPS_MODIFY_KEY_FLAG: char = '*';
const HALF_KEY_SEPARATOR: char = ':';

const LEFTSHIFT_INDEX: usize = KEY_LEFTSHIFT as usize;
const RIGHTSHIFT_INDEX: usize = KEY_RIGHTSHIFT as usize;
const CAPSLOCK_INDEX: usize = KEY_CAPSLOCK as usize;

const KEY_LEFTSHIFT_U16: u16 = KEY_LEFTSHIFT as u16;
const KEY_RIGHTSHIFT_U16: u16 = KEY_RIGHTSHIFT as u16;
const KEY_CAPSLOCK_U16: u16 = KEY_CAPSLOCK as u16;

trait KeyMapper {
    fn send_event(&self, key_state: &[bool], event: &mut input_event, device: &Device) -> Result<()>;
}

pub struct KeyMaps {
    keymaps: Vec<Box<KeyMapper>>,
    keymap_index_keys: HashMap<u16, usize>,
    switch_layout_keys: Vec<usize>,
    key_state: [bool; KEY_MAX],
    revert_default_key: u16,
    revert_keymap_index: usize,
    // above do not change, below does
    chosen_keymap_index: usize,
    current_keymap_index: usize,
}

fn parse_key(key_map: &HashMap<&'static str, c_int>, key: &str) -> u16 {
    match key_map.get(key.trim_matches(|c: char| c.is_whitespace() || c == INVERT_KEY_FLAG || c == CAPS_MODIFY_KEY_FLAG)) {
        Some(key_code) => *key_code as u16,
        None => panic!("unknown key: {}", key.trim())
    }
}

fn parse_keymap_numeric(key_map: &HashMap<&'static str, c_int>, keymap: &str) -> Vec<u16> {
    keymap.split(",").map(|k| parse_key(key_map, k)).collect()
}

fn parse_key_half_inverted(key_map: &HashMap<&'static str, c_int>, key: &str) -> HalfInvertedKey {
    HalfInvertedKey {
        code: parse_key(key_map, key),
        invert_shift: key.contains(INVERT_KEY_FLAG),
        capslock_nomodify: key.contains(CAPS_MODIFY_KEY_FLAG),
    }
}

// maybe shortcut to this if not contains * or :
fn parse_keymap_u16(key_map: &HashMap<&'static str, c_int>, keymap: &str) -> Vec<u16> {
    keymap.split(",").map(|k| parse_key(key_map, k)).collect()
}

// todo: how do I return an iterator here instead of .collect to Vec?
fn parse_keymap(key_map: &HashMap<&'static str, c_int>, keymap: &str) -> Vec<Key> {
    keymap.split(",").map(|k| {
        let ret: Key = if k.contains(HALF_KEY_SEPARATOR) {
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

impl KeyMaps {
    pub fn from_cfg<P: AsRef<Path>>(key_map: &HashMap<&'static str, c_int>, path: P) -> KeyMaps {
        let key_map_config = parse_cfg(path).expect("provided config cannot be found/parsed");
        KeyMaps::new(key_map, key_map_config)
    }

    pub fn new(key_map: &HashMap<&'static str, c_int>, config: KeymapConfig) -> KeyMaps {
        if config.keymaps.len() < 2 {
            panic!("must have at least 2 keymaps (original and mapped) but only have {},", config.keymaps.len());
        }
        if config.default_keymap_index >= config.keymaps.len() || config.revert_keymap_index >= config.keymaps.len() {
            panic!("default_keymap_index ({}) and revert_keymap_index ({}) must be less than keymaps length ({}),", config.default_keymap_index, config.revert_keymap_index, config.keymaps.len());
        }
        let base_keymap = parse_keymap_numeric(key_map, &config.keymaps[0]);
        //println!("base_keymap      : {:?}", base_keymap);
        let mut keymaps: Vec<Box<KeyMapper>> = vec!(Box::new(Key::Noop)); // todo: can we share the box?
        let mut keymap_index_keys: HashMap<u16, usize> = HashMap::new();
        for (x, v) in config.keymaps.iter().enumerate() {
            keymap_index_keys.insert(*key_map.get(&*x.to_string()).unwrap() as u16, x);
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

        KeyMaps {
            keymaps: keymaps,
            keymap_index_keys: keymap_index_keys,
            switch_layout_keys: config.switch_layout_keys.iter().map(|k| parse_key(key_map, k) as usize).collect(),
            key_state: [false; KEY_MAX],
            // todo: detect key state? at least CAPSLOCK...
            revert_default_key: parse_key(key_map, &config.revert_default_key),
            revert_keymap_index: config.revert_keymap_index,
            chosen_keymap_index: config.default_keymap_index,
            current_keymap_index: config.default_keymap_index,
        }
    }
}

//impl KeyMapper for KeyMaps {
impl KeyMaps {
    pub fn send_event(&mut self, mut event: &mut input_event, device: &Device) -> Result<()> {
        //println!("type: {} code: {} value: {}", event.type_, event.code, event.value);
        if event.value != 2 {
            // todo: index check here...
            if event.code == KEY_CAPSLOCK_U16 {
                if event.value == DOWN {
                    self.key_state[CAPSLOCK_INDEX] = !self.key_state[CAPSLOCK_INDEX];
                }
            } else {
                self.key_state[event.code as usize] = event.value == DOWN;
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
                let new_index = self.keymap_index_keys.get(&event.code);
                if new_index.is_some() {
                    self.chosen_keymap_index = *new_index.unwrap();
                    self.current_keymap_index = self.chosen_keymap_index; // todo: what if revert_default_key is held? for now ignore
                    return Ok(()); // we don't want to also send this keypress, so bail
                }
            }
            if event.code == self.revert_default_key {
                match event.value {
                    // todo: ctrl+c will get c stuck because code c value 1 will be sent, but then we'll let go of ctrl, and code j value 0 is sent, so c is never released... fix that...
                    DOWN => self.current_keymap_index = self.revert_keymap_index,
                    UP => self.current_keymap_index = self.chosen_keymap_index,
                    _ => () // do nothing for 2
                }
            }
        }
        self.keymaps[self.current_keymap_index].send_event(&self.key_state, &mut event, device)
    }
}

// 249 is one more than KEY_MICMUTE which is max key in uinput-sys event.rs
const KEY_MAX: usize = 249;

struct KeyMap {
    //keymap: Vec<Key>,
    keymap: [Key; KEY_MAX],
}

impl KeyMap {
    pub fn new() -> Self {
        //let mut keymap = [0u16; KEY_MAX];
        //let mut keymap : [Box<KeyMapper>; KEY_MAX] = [Box::new(NOOP); KEY_MAX];
        //let mut keymap : [Box<KeyMapper>; KEY_MAX] = [Box::new(0u16); KEY_MAX];
        let keymap : [Key; KEY_MAX] = [Key::Noop; KEY_MAX];
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
    pub fn map(&mut self, from: u16, to: Key) {
        self.keymap[from as usize] = to;
    }
}

impl KeyMapper for KeyMap {
    fn send_event(&self, key_state: &[bool], event: &mut input_event, device: &Device) -> Result<()> {
        self.keymap[event.code as usize].send_event(key_state, event, device)
    }
}

struct CodeKeyMap {
    //keymap: Vec<Key>,
    keymap: [u16; KEY_MAX],
}

impl CodeKeyMap {
    pub fn new() -> Self {
        let mut keymap = [0u16; KEY_MAX];
        // which is rustier
        /*
        for x  in 0..KEY_MAX {
            keymap[x as usize] = x as u16;
        }
        */
        for (x, v) in keymap.iter_mut().enumerate() {
            *v = x as u16;
        }
        //println!("keymap: {:?}", &keymap[..]);
        CodeKeyMap {
            keymap: keymap
        }
    }

    pub fn map(&mut self, from: u16, to: u16) {
        self.keymap[from as usize] = to;
    }
}

impl KeyMapper for CodeKeyMap {
    fn send_event(&self, key_state: &[bool], event: &mut input_event, device: &Device) -> Result<()> {
        self.keymap[event.code as usize].send_event(key_state, event, device)
    }
}

#[allow(unused_variables, unused_mut)]
impl KeyMapper for u16 {
    fn send_event(&self, key_state: &[bool], mut event: &mut input_event, device: &Device) -> Result<()> {
        event.code = *self;
        device.write_event(event)
    }
}

// todo:capslock_nomodify is like a whole-key thing, not a half-key thing, split code/invert_shift to own struct, send into send_key from *InvertedKey, maybe anyway, consider it, maybe 1 char for whole key and another for half?
#[derive(Clone, Copy)]
struct HalfInvertedKey {
    code: u16,
    // code this is describing
    invert_shift: bool,
    // true to invert shift for this code
    capslock_nomodify: bool,
    // true means capslock does not normally modify this, but you would like it to
}

impl HalfInvertedKey {
    fn send_key(&self, key_state: &[bool], event: &mut input_event, device: &Device, left_shift: bool, right_shift: bool, caps_lock: bool) -> Result<()> {
        let code = self.code;
        let value = event.value;
        let mut invert_shift = self.invert_shift;
        if value == DOWN {
            if caps_lock && self.capslock_nomodify {
                invert_shift = !invert_shift;
            }
            if invert_shift {
                if left_shift {
                    event.code = KEY_LEFTSHIFT_U16;
                    event.value = UP;
                } else if right_shift {
                    event.code = KEY_RIGHTSHIFT_U16;
                    event.value = UP;
                } else {
                    event.code = KEY_LEFTSHIFT_U16;
                    event.value = DOWN;
                }
                //event.code.send_event(key_state, event, device);
                device.write_event(event)?;
                // SYN_REPORT after, then key, then key's SYN_REPORT
                device.synchronize()?;
                event.code = code; // not needed since u16 does it
                event.value = value;
            }
        }
        code.send_event(key_state, event, device)?;
        if value == UP {
            if caps_lock && self.capslock_nomodify {
                invert_shift = !invert_shift;
            }
            if invert_shift {
                if left_shift {
                    event.code = KEY_LEFTSHIFT_U16;
                    event.value = DOWN;
                } else if right_shift {
                    event.code = KEY_RIGHTSHIFT_U16;
                    event.value = DOWN;
                } else {
                    event.code = KEY_LEFTSHIFT_U16;
                    event.value = UP;
                }
                //event.code.send_event(key_state, event, device);
                // SYN_REPORT first after key, then shift, then key's SYN_REPORT which will be used for shift's
                device.synchronize()?;
                device.write_event(event)?;
                // neither of these are needed now...
                event.code = code; // not needed since u16 does it
                event.value = value;
            }
        }
        Ok(())
    }
}

impl KeyMapper for HalfInvertedKey {
    fn send_event(&self, key_state: &[bool], event: &mut input_event, device: &Device) -> Result<()> {
        let left_shift = key_state[LEFTSHIFT_INDEX];
        let right_shift = key_state[RIGHTSHIFT_INDEX];
        let caps_lock = key_state[CAPSLOCK_INDEX];
        self.send_key(key_state, event, device, left_shift, right_shift, caps_lock)
    }
}

#[derive(Clone, Copy)]
enum Key {
    Noop,
    Direct(u16),
    HalfKey(HalfInvertedKey),
    FullKey(HalfInvertedKey, HalfInvertedKey),
}

impl KeyMapper for Key {
    fn send_event(&self, key_state: &[bool], event: &mut input_event, device: &Device) -> Result<()> {
        match *self {
            Key::Noop => {
                device.write_event(event)
            },
            Key::Direct(code) => {
                code.send_event(key_state, event, device)
            },
            Key::HalfKey(ref key_half) => {
                key_half.send_event(key_state, event, device)
            },
            Key::FullKey(ref noshift_half, ref shift_half) => {
                let left_shift = key_state[LEFTSHIFT_INDEX];
                let right_shift = key_state[RIGHTSHIFT_INDEX];
                let caps_lock = key_state[CAPSLOCK_INDEX];
                if caps_lock != (left_shift || right_shift) {
                    shift_half.send_key(key_state, event, device, left_shift, right_shift, caps_lock)
                } else {
                    noshift_half.send_key(key_state, event, device, left_shift, right_shift, caps_lock)
                }
            },
        }
    }
}

use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct KeymapConfig {
    switch_layout_keys: Vec<String>,
    revert_default_key: String,
    revert_keymap_index: usize,
    default_keymap_index: usize,
    keymaps: Vec<String>
}

fn parse_cfg<P: AsRef<Path>>(path: P) -> Result<KeymapConfig> {
    let mut f = File::open(path)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    //toml::from_str(&input)?
    match toml::from_str(&input) {
        Ok(toml) => Ok(toml),
        Err(_) => Err(Error::NotFound) // todo: something better
    }
}
