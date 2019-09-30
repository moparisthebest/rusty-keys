use std::{mem, ptr, slice};
use libc::{timeval, gettimeofday, input_event, c_int};
use nix::{unistd, errno::Errno};
use ffi::*;
use {Result as Res};

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
	pub fn write(&self, kind: c_int, code: c_int, value: c_int) -> Res<()> {
		let mut event = input_event {
			time:  timeval { tv_sec: 0, tv_usec: 0 },
			type_:  kind as u16,
			code:  code as u16,
			value: value as i32,
		};

		self.write_event(&mut event)
	}

	#[doc(hidden)]
	pub fn write_event(&self, event: &mut input_event) -> Res<()> {
		unsafe {
			gettimeofday(&mut event.time, ptr::null_mut());

			let ptr  = event as *const _ as *const u8;
			let size = mem::size_of_val(event);

			try!(unistd::write(self.fd, slice::from_raw_parts(ptr, size)));
		}

		Ok(())
	}

	/// Synchronize the device.
	pub fn synchronize(&self) -> Res<()> {
		self.write(EV_SYN, SYN_REPORT, 0)
	}

	/// Send an event.
	pub fn send(&self, kind: c_int, code: c_int, value: i32) -> Res<()> {
		self.write(kind, code, value)
	}

	/// Send a press event.
	pub fn press(&self, kind: c_int, code: c_int) -> Res<()> {
		self.write(kind, code, 1)
	}

	/// Send a release event.
	pub fn release(&self, kind: c_int, code: c_int) -> Res<()> {
		self.write(kind, code, 0)
	}

	/// Send a press and release event.
	pub fn click(&self, kind: c_int, code: c_int) -> Res<()> {
		try!(self.press(kind, code));
		try!(self.release(kind, code));

		Ok(())
	}

	/// Send a relative or absolute positioning event.
	pub fn position(&self, kind: c_int, code: c_int, value: i32) -> Res<()> {
		self.write(kind, code, value)
	}
}

impl Drop for Device {
	fn drop(&mut self) {
		unsafe {
			Errno::result(ui_dev_destroy(self.fd)).unwrap();
		}
	}
}
