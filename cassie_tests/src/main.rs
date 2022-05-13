use std::io::Read;
use gag::BufferRedirect;
fn main() {
    let mut buf = BufferRedirect::stdout().unwrap();
    println!("Hello world!");
    println!("Hello world1!");
    println!("Hello world2!");

    let mut output = String::new();
    buf.read_to_string(&mut output).unwrap();
    drop(buf);
    println!("console:{}", &output[..]);
}
