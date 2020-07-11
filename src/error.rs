use std::fmt;
use std::error;
use std::ffi;
use std::io;

#[cfg(target_os = "linux")]
use std::sync::mpsc;

#[cfg(target_os = "linux")]
use nix;

#[cfg(target_os = "linux")]
use libc;

/// UInput error.
#[derive(Debug)]
pub enum Error {
	/// System errors.
	#[cfg(target_os = "linux")]
	Nix(nix::Error),

	/// Errors with internal nulls in names.
	Nul(ffi::NulError),

	Io(io::Error),

	#[cfg(target_os = "linux")]
	Send(mpsc::SendError<libc::input_event>),

	/// The uinput file could not be found.
	NotFound,

	/// error reading input_event
	ShortRead,
}

impl From<ffi::NulError> for Error {
	fn from(value: ffi::NulError) -> Self {
		Error::Nul(value)
	}
}

#[cfg(target_os = "linux")]
impl From<nix::Error> for Error {
	fn from(value: nix::Error) -> Self {
		Error::Nix(value)
	}
}

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Error::Io(value)
	}
}

#[cfg(target_os = "linux")]
impl From<mpsc::SendError<libc::input_event>> for Error {
	fn from(value: mpsc::SendError<libc::input_event>) -> Self {
		Error::Send(value)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			#[cfg(target_os = "linux")]
			&Error::Nix(ref err) => err.fmt(f),

			&Error::Nul(ref err) => err.fmt(f),

			&Error::Io(ref err) => err.fmt(f),

			#[cfg(target_os = "linux")]
			&Error::Send(ref err) => err.fmt(f),

			&Error::NotFound => f.write_str("Device not found."),

			&Error::ShortRead => f.write_str("Error while reading from device file."),
		}
	}
}

impl error::Error for Error {}
