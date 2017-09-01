extern crate uinput;
extern crate uinput_sys as ffi;

use ffi::*;

use std::thread;
use std::time::Duration;

fn main() {
	let mut device = uinput::default().expect("1")
		.name("test").expect("2")
		.event().expect("3")
		//.event(uinput::event::Keyboard::All).unwrap()
		.create().expect("4");

	thread::sleep(Duration::from_secs(1));

	device.click(EV_KEY, KEY_H).unwrap();
	/*
	device.click(&keyboard::Key::H).unwrap();
	device.click(&keyboard::Key::E).unwrap();
	device.click(&keyboard::Key::L).unwrap();
	device.click(&keyboard::Key::L).unwrap();
	device.click(&keyboard::Key::O).unwrap();
	device.click(&keyboard::Key::Space).unwrap();
	device.click(&keyboard::Key::W).unwrap();
	device.click(&keyboard::Key::O).unwrap();
	device.click(&keyboard::Key::R).unwrap();
	device.click(&keyboard::Key::L).unwrap();
	device.click(&keyboard::Key::D).unwrap();
	device.click(&keyboard::Key::Enter).unwrap();
	*/

	device.synchronize().unwrap();
}
