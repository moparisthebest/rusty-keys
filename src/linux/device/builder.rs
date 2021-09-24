use std::path::Path;
use std::{mem, slice};
use std::ffi::CString;
use libc::c_int;
use nix::{self, fcntl, unistd, ioctl_write_ptr, ioctl_none};
use nix::sys::stat;
//use uinput_sys::*;
use crate::{Result as Res, Device};
use std::collections::hash_map::Values;
use std::os::raw::c_char;

use crate::linux::device::codes::*;

/*
uin!(write ui_set_evbit   with b'U', 100; c_int);
uin!(write ui_set_keybit  with b'U', 101; c_int);

ioctl!(none ui_dev_create with b'U', 1);

ioctl!(none ui_dev_destroy with b'U', 2);
*/

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
	pub fn open<P: AsRef<Path>>(path: P) -> Res<Self> {
		Ok(Builder {
			fd:  fcntl::open(path.as_ref(), fcntl::OFlag::O_WRONLY | fcntl::OFlag::O_NONBLOCK, stat::Mode::empty())?,
			def: unsafe { mem::zeroed() },
			abs: None,
		})
	}

	/// Create a builder from `/dev/uinput`.
	pub fn default() -> Res<Self> {
		Builder::open("/dev/uinput")
	}

	/// Set the name.
	pub fn name<T: AsRef<str>>(mut self, value: T) -> Res<Self> {
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

	pub fn event(mut self, key_codes: Values<&str, u16>) -> Res<Self> {
		self.abs = None;
		//let test_ev_key : c_int = EV_KEY as c_int;
		unsafe {
			//try!(Errno::result(ui_set_evbit(self.fd, EV_KEY)));
			//try!(Errno::result(ui_set_keybit(self.fd, KEY_H)));

			//Errno::result(ui_set_evbit(self.fd, EV_KEY as *const c_int))?;

			ui_set_evbit(self.fd, EV_KEY as *const c_int)?;

			//ui_set_keybit(self.fd, KEY_H as *const c_int)?;
			for key_code in key_codes {
				ui_set_keybit(self.fd, *key_code as *const c_int)?;
			}
			//try!(ui_set_keybit(self.fd, &KEY_H));
		}
		Ok(self)
	}
/*
	/// Enable the given event.
	pub fn event<T: Into<Event>>(mut self, value: T) -> Res<Self> {
		self.abs = None;

		match value.into() {
			Event::All => {
				try!(self.event(Event::Keyboard(event::Keyboard::All)))
					.event(Event::Controller(event::Controller::All))
			}

			Event::Keyboard(value) => {
				match value {
					event::Keyboard::All => {
						let mut builder = self;

						for item in event::keyboard::Key::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::KeyPad::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::Misc::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::InputAssist::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::Function::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::Braille::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::Numeric::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::TouchPad::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::Camera::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::Attendant::iter_variants() {
							builder = try!(builder.event(item));
						}

						Ok(builder)
					}

					value => {
						unsafe {
							try!(Errno::result(ui_set_evbit(self.fd, value.kind())));
							try!(Errno::result(ui_set_keybit(self.fd, value.code())));
						}

						Ok(self)
					}
				}
			}

			Event::Controller(value) => {
				match value {
					event::Controller::All => {
						let mut builder = self;

						for item in event::controller::Misc::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::Mouse::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::JoyStick::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::GamePad::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::Digi::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::Wheel::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::DPad::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::TriggerHappy::iter_variants() {
							builder = try!(builder.event(item));
						}

						Ok(builder)
					}

					value => {
						unsafe {
							try!(Errno::result(ui_set_evbit(self.fd, value.kind())));
							try!(Errno::result(ui_set_keybit(self.fd, value.code())));
						}

						Ok(self)
					}
				}
			}

			Event::Relative(value) => {
				unsafe {
					try!(Errno::result(ui_set_evbit(self.fd, value.kind())));
					try!(Errno::result(ui_set_relbit(self.fd, value.code())));
				}

				Ok(self)
			}

			Event::Absolute(value) => {
				unsafe {
					try!(Errno::result(ui_set_evbit(self.fd, value.kind())));
					try!(Errno::result(ui_set_absbit(self.fd, value.code())));
				}

				self.abs = Some(value.code());

				Ok(self)
			}
		}
	}
*/
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
	pub fn create(self) -> Res<Device> {
		unsafe {
			let ptr  = &self.def as *const _ as *const u8;
			let size = mem::size_of_val(&self.def);

			unistd::write(self.fd, slice::from_raw_parts(ptr, size))?;
			//todo: try!(Errno::result(ui_dev_create(self.fd)));
			// try1: Errno::result(ui_dev_create(self.fd)).unwrap();
			ui_dev_create(self.fd)?;
		}

		Ok(Device::new(self.fd))
	}
}
