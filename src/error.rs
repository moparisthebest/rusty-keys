use std::fmt;
use std::error;
use std::ffi;
use std::io;

#[cfg(target_os = "linux")]
use nix;

/// UInput error.
#[derive(Debug)]
pub enum Error {
	/// System errors.
	#[cfg(target_os = "linux")]
	Nix(nix::Error),

	/// Errors with internal nulls in names.
	Nul(ffi::NulError),

	Io(io::Error),
	
	Toml(toml::de::Error),

	NotAKeyboard,
	
	/// error reading input_event
	ShortRead,

	/// epoll already added
	EpollAlreadyAdded,
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

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			#[cfg(target_os = "linux")]
			&Error::Nix(ref err) => err.fmt(f),

			&Error::Nul(ref err) => err.fmt(f),

			&Error::Io(ref err) => err.fmt(f),
			
			&Error::Toml(ref err) => err.fmt(f),
			
			&Error::NotAKeyboard => f.write_str("This device file is not a keyboard"),

			&Error::ShortRead => f.write_str("Error while reading from device file."),
			
			&Error::EpollAlreadyAdded => f.write_str("epoll already added, delete first"),
		}
	}
}

impl error::Error for Error {}
