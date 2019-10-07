
#[cfg(any(target_os = "windows", target_os = "linux"))]
fn main() {
    let ret = rusty_keys::main_res();
    if let Err(e) = ret {
        println!("fatal error: {}", e);
    }
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn main() {
    panic!("sorry no main impl for this platform");
}
