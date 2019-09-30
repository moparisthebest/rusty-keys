use std::mem;
use std::fs::File;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use libc::{input_event, c_int};
use {Error,Result};

ioctl_write_ptr!(eviocgrab, b'E', 0x90, c_int);

// TODO: use size_of_input_event instead of hard-coding 24.
const SIZE_OF_INPUT_EVENT: usize = 24;//mem::size_of::<input_event>();

pub struct InputDevice {
    device_file: File,
    buf: [u8; SIZE_OF_INPUT_EVENT],
}

impl InputDevice {
    pub fn open(device_file: &str) -> Result<Self> {
        let device_file = File::open(device_file)?;
        Ok(InputDevice {
            device_file: device_file,
            buf: [0u8; SIZE_OF_INPUT_EVENT],
        })
    }

    pub fn read_event(&mut self) -> Result<input_event> {
        let num_bytes = self.device_file.read(&mut self.buf)?;
        if num_bytes != SIZE_OF_INPUT_EVENT {
            return Err(Error::ShortRead);
        }
        let event: input_event = unsafe { mem::transmute(self.buf) };
        Ok(event)
    }

    pub fn grab(&mut self) -> Result<()> {
        unsafe {
            eviocgrab(self.device_file.as_raw_fd(), 1 as *const c_int)?;
        }
        Ok(())
    }

    pub fn release(&mut self) -> Result<()> {
        unsafe {
            eviocgrab(self.device_file.as_raw_fd(), 0 as *const c_int)?;
        }
        Ok(())
    }
}

impl Drop for InputDevice {
    fn drop(&mut self) {
        self.release().ok(); // ignore any errors here, what could we do anyhow?
    }
}
