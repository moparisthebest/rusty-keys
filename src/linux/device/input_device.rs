use std::mem;
use std::fs::File;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use std::os::unix::prelude::RawFd;
use libc::{input_event, c_int};
use nix::{ioctl_write_ptr, ioctl_read_buf};
use nix::fcntl::{OFlag, fcntl, FcntlArg};

#[cfg(feature = "epoll_inotify")]
use epoll::ControlOptions::{EPOLL_CTL_ADD, EPOLL_CTL_DEL};

use crate::{Error,Result};
use crate::linux::{EV_KEY, KEY_MAX, NAME, KEY_W, KEY_A, KEY_S, KEY_D};

ioctl_write_ptr!(eviocgrab, b'E', 0x90, c_int);
ioctl_read_buf!(eviocgname, b'E', 0x06, u8);
ioctl_read_buf!(eviocgbit, b'E', 0x20, u8);
ioctl_read_buf!(eviocgbit_ev_key, b'E', 0x20 + EV_KEY, u8);

const SIZE_OF_INPUT_EVENT: usize = mem::size_of::<input_event>();

pub struct InputDevice {
    device_file: File,
    grabbed: bool,
    epoll_fd: Option<RawFd>,
}

impl InputDevice {
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        Ok(InputDevice {
            device_file: File::open(path)?,
            grabbed: false,
            epoll_fd: None,
        })
    }
    
    pub fn new_input_event_buf() -> [u8; SIZE_OF_INPUT_EVENT] {
        [0u8; SIZE_OF_INPUT_EVENT]
    }

    pub fn read_event(&mut self, buf: &mut [u8; SIZE_OF_INPUT_EVENT]) -> Result<input_event> {
        let num_bytes = self.device_file.read(buf)?;
        if num_bytes != SIZE_OF_INPUT_EVENT {
            return Err(Error::ShortRead);
        }
        let event: input_event = unsafe { mem::transmute(*buf) };
        Ok(event)
    }

    pub fn valid_keyboard_device(self) -> Result<Self> {
        use std::os::unix::fs::FileTypeExt;

        // must be a character device
        if !self.device_file.metadata()?.file_type().is_char_device() {
            return Err(Error::NotAKeyboard);
        }
        
        let raw_fd = self.device_file.as_raw_fd();

        // does it support EV_KEY
        let mut evbit = [0u8; 8];
        unsafe {
            eviocgbit(raw_fd, &mut evbit)?;
        };
        let evbit = u64::from_ne_bytes(evbit);
        if (evbit & (1 << EV_KEY)) == 0 {
            return Err(Error::NotAKeyboard);
        }

        // does it support all keys WASD ? (yes this is fairly random but probably good enough, could make configuration probably)
        let mut key_bits = [0u8; (KEY_MAX as usize / 8) + 1];
        unsafe {
            eviocgbit_ev_key(raw_fd, &mut key_bits)?;
        };
        let key_unsupported = |key : c_int| (key_bits[key as usize / 8] & (1 << (key % 8))) == 0;
        if key_unsupported(KEY_W) || key_unsupported(KEY_A) || key_unsupported(KEY_S) || key_unsupported(KEY_D) {
            return Err(Error::NotAKeyboard);
        }

        // is it another running copy of rusty-keys ?
        let mut name = [0u8; NAME.len()];
        unsafe {
            eviocgname(raw_fd, &mut name)?
        };
        if NAME.as_bytes() == &name {
            return Err(Error::NotAKeyboard);
        }
        return Ok(self);
    }

    pub fn grab(mut self) -> Result<Self> {
        unsafe {
            eviocgrab(self.device_file.as_raw_fd(), 1 as *const c_int)?;
        }
        self.grabbed = true;
        Ok(self)
    }

    pub fn release(&mut self) -> Result<()> {
        if self.grabbed {
            unsafe {
                eviocgrab(self.device_file.as_raw_fd(), 0 as *const c_int)?;
            }
            self.grabbed = false;
        }
        Ok(())
    }
    
    #[cfg(feature = "epoll_inotify")]
    pub fn epoll_add(mut self, epoll_fd: RawFd, data: u64) -> Result<Self> {
        if None != self.epoll_fd {
            return Err(Error::EpollAlreadyAdded);
        }
        let raw_fd = self.device_file.as_raw_fd();
        let flags = unsafe {
            // https://github.com/nix-rust/nix/issues/1102
            OFlag::from_bits_unchecked(fcntl(raw_fd, FcntlArg::F_GETFL)?)
        };
        fcntl(raw_fd, FcntlArg::F_SETFL(flags | OFlag::O_NONBLOCK))?;

        let epoll_event = epoll::Event::new(epoll::Events::EPOLLIN | epoll::Events::EPOLLET, data);
        epoll::ctl(epoll_fd, EPOLL_CTL_ADD, raw_fd, epoll_event)?;
        self.epoll_fd = Some(epoll_fd);
        Ok(self)
    }

    #[cfg(feature = "epoll_inotify")]
    pub fn epoll_del(&mut self) -> Result<&mut Self> {
        if let Some(epoll_fd) = self.epoll_fd {
            // set this to None first, if we end up returning an Err early, we can't do anything else anyway...
            self.epoll_fd = None;
            let empty_event = epoll::Event::new(epoll::Events::empty(), 0);
            epoll::ctl(epoll_fd, EPOLL_CTL_DEL, self.device_file.as_raw_fd(), empty_event)?;
        }
        Ok(self)
    }
}

impl Drop for InputDevice {
    fn drop(&mut self) {
        // ignore any errors here, what could we do anyhow?
        self.release().ok();
        #[cfg(feature = "epoll_inotify")]
        self.epoll_del().ok();
    }
}
