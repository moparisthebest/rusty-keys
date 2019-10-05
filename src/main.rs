use rusty_keys::main_res;

fn main() {
    let ret = main_res();
    if let Err(e) = ret {
        println!("fatal error: {}", e);
    }
}
