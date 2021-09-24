use std::{mem, ptr, slice};
use libc::{timeval, gettimeofday, input_event, c_int};
use nix::{unistd, ioctl_none};
use crate::Result;

use crate::linux::device::codes::*;

ioctl_none!(ui_dev_destroy, b'U', 2);

/// The virtual device.
pub struct Device {
	fd: c_int,
}

impl Device {
	/// Wrap a file descriptor in a `Device`.
	pub fn new(fd: c_int) -> Self {
		Device {
			fd: fd
		}
	}

	#[doc(hidden)]
	pub fn write(&self, kind: c_int, code: c_int, value: c_int) -> Result<()> {
		let mut event = input_event {
			time:  timeval { tv_sec: 0, tv_usec: 0 },
			type_:  kind as u16,
			code:  code as u16,
			value: value as i32,
		};

		self.write_event(&mut event)
	}

	#[doc(hidden)]
	pub fn write_event(&self, event: &mut input_event) -> Result<()> {
		unsafe {
			gettimeofday(&mut event.time, ptr::null_mut());

			let ptr  = event as *const _ as *const u8;
			let size = mem::size_of_val(event);

			unistd::write(self.fd, slice::from_raw_parts(ptr, size))?;
		}

		Ok(())
	}

	/// Synchronize the device.
	pub fn synchronize(&self) -> Result<()> {
		self.write(EV_SYN, SYN_REPORT, 0)
	}

	/// Send an event.
	pub fn send(&self, kind: c_int, code: c_int, value: i32) -> Result<()> {
		self.write(kind, code, value)
	}

	/// Send a press event.
	pub fn press(&self, kind: c_int, code: c_int) -> Result<()> {
		self.write(kind, code, 1)
	}

	/// Send a release event.
	pub fn release(&self, kind: c_int, code: c_int) -> Result<()> {
		self.write(kind, code, 0)
	}

	/// Send a press and release event.
	pub fn click(&self, kind: c_int, code: c_int) -> Result<()> {
		self.press(kind, code)?;
		self.release(kind, code)
	}

	/// Send a relative or absolute positioning event.
	pub fn position(&self, kind: c_int, code: c_int, value: i32) -> Result<()> {
		self.write(kind, code, value)
	}
}

impl Drop for Device {
	fn drop(&mut self) {
		unsafe {
			// ignore error here so as to not panic in a drop
			ui_dev_destroy(self.fd).ok();
		}
	}
}
