#![recursion_limit = "1000"]

extern crate libc;

extern crate uinput_sys as ffi;

#[macro_use]
extern crate nix;

#[macro_use]
extern crate serde_derive;

use std::path::Path;

mod error;
pub use error::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

pub mod keymapper;
pub use keymapper::KeyMaps;

pub mod device;
pub use device::{Device,InputDevice};

/// Open the default uinput device.
pub fn default() -> Result<device::Builder> {
	device::Builder::default()
}

/// Open the specified uinput device.
pub fn open<P: AsRef<Path>>(path: P) -> Result<device::Builder> {
	device::Builder::open(path)
}
