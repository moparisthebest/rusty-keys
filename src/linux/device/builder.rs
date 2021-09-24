use std::path::Path;
use std::{mem, slice};
use std::ffi::CString;
use libc::c_int;
use nix::{self, fcntl, unistd, ioctl_write_ptr, ioctl_none};
use nix::sys::stat;
use crate::{Result, Device};
use std::collections::hash_map::Values;
use std::os::raw::c_char;

use crate::linux::device::codes::*;

ioctl_write_ptr!(ui_set_evbit, b'U', 100, c_int);
ioctl_write_ptr!(ui_set_keybit, b'U', 101, c_int);
ioctl_none!(ui_dev_create, b'U', 1);

pub const UINPUT_MAX_NAME_SIZE: c_int = 80;
pub const ABS_MAX: c_int = 0x3f;
pub const ABS_CNT: c_int = ABS_MAX + 1;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct input_id {
	pub bustype: u16,
	pub vendor:  u16,
	pub product: u16,
	pub version: u16,
}

#[repr(C)]
pub struct uinput_user_dev {
	pub name: [c_char; UINPUT_MAX_NAME_SIZE as usize],
	pub id:   input_id,

	pub ff_effects_max: u32,
	pub absmax:  [i32; ABS_CNT as usize],
	pub absmin:  [i32; ABS_CNT as usize],
	pub absfuzz: [i32; ABS_CNT as usize],
	pub absflat: [i32; ABS_CNT as usize],
}

/// Device builder.
pub struct Builder {
	fd:  c_int,
	def: uinput_user_dev,
	abs: Option<c_int>,
}

impl Builder {
	/// Create a builder from the specified path.
	pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
		Ok(Builder {
			fd:  fcntl::open(path.as_ref(), fcntl::OFlag::O_WRONLY | fcntl::OFlag::O_NONBLOCK, stat::Mode::empty())?,
			def: unsafe { mem::zeroed() },
			abs: None,
		})
	}

	/// Create a builder from `/dev/uinput`.
	pub fn default() -> Result<Self> {
		Builder::open("/dev/uinput")
	}

	/// Set the name.
	pub fn name<T: AsRef<str>>(mut self, value: T) -> Result<Self> {
		let string = CString::new(value.as_ref())?;
		let bytes  = string.as_bytes_with_nul();

		if bytes.len() > UINPUT_MAX_NAME_SIZE as usize {
			Err(nix::Error::EINVAL)?;
		}

		(&mut self.def.name)[..bytes.len()]
			.clone_from_slice(unsafe { mem::transmute(bytes) });

		Ok(self)
	}

	/// Set the bus type.
	pub fn bus(mut self, value: u16) -> Self {
		self.def.id.bustype = value;
		self
	}

	/// Set the vendor ID.
	pub fn vendor(mut self, value: u16) -> Self {
		self.def.id.vendor = value;
		self
	}

	/// Set the product ID.
	pub fn product(mut self, value: u16) -> Self {
		self.def.id.product = value;
		self
	}

	/// Set the version.
	pub fn version(mut self, value: u16) -> Self {
		self.def.id.version = value;
		self
	}

	pub fn event(mut self, key_codes: Values<&str, u16>) -> Result<Self> {
		self.abs = None;
		unsafe {
			ui_set_evbit(self.fd, EV_KEY as *const c_int)?;

			for key_code in key_codes {
				ui_set_keybit(self.fd, *key_code as *const c_int)?;
			}
		}
		Ok(self)
	}

	/// Set the maximum value for the previously enabled absolute event.
	pub fn max(mut self, value: i32) -> Self {
		self.def.absmax[self.abs.unwrap() as usize] = value;
		self
	}

	/// Set the minimum value for the previously enabled absolute event.
	pub fn min(mut self, value: i32) -> Self {
		self.def.absmin[self.abs.unwrap() as usize] = value;
		self
	}

	/// Set the fuzz value for the previously enabled absolute event.
	pub fn fuzz(mut self, value: i32) -> Self {
		self.def.absfuzz[self.abs.unwrap() as usize] = value;
		self
	}

	/// Set the flat value for the previously enabled absolute event.
	pub fn flat(mut self, value: i32) -> Self {
		self.def.absflat[self.abs.unwrap() as usize] = value;
		self
	}

	/// Create the defined device.
	pub fn create(self) -> Result<Device> {
		unsafe {
			let ptr  = &self.def as *const _ as *const u8;
			let size = mem::size_of_val(&self.def);

			unistd::write(self.fd, slice::from_raw_parts(ptr, size))?;
			ui_dev_create(self.fd)?;
		}

		Ok(Device::new(self.fd))
	}
}
