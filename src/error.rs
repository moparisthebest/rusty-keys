use std::fmt;
use std::error;
use std::ffi;
use std::io;
use std::sync::mpsc;
use nix;

use libc;

/// UInput error.
#[derive(Debug)]
pub enum Error {
	/// System errors.
	Nix(nix::Error),

	/// Errors with internal nulls in names.
	Nul(ffi::NulError),

	Io(io::Error),

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

impl From<mpsc::SendError<libc::input_event>> for Error {
	fn from(value: mpsc::SendError<libc::input_event>) -> Self {
		Error::Send(value)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		f.write_str(error::Error::description(self))
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match self {
			&Error::Nix(ref err) =>
				err.description(),

			&Error::Nul(ref err) =>
				err.description(),

			&Error::Io(ref err) =>
				err.description(),

			&Error::Send(ref err) =>
				err.description(),

			&Error::NotFound =>
				"Device not found.",

			&Error::ShortRead =>
				"Error while reading from device file.",
		}
	}
}
