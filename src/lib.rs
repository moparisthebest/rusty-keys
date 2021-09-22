#![recursion_limit = "1000"]

pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub mod error;
pub use error::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

pub mod keymapper;
pub use keymapper::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;
