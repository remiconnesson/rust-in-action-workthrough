use std::thread;

// this code doesn't compile, run to read compiler's messages
fn main() {
    let mut data = 100;

    thread::spawn(|| data = 500);
    thread::spawn(|| data = 1000);

    println!("{}", data);
}
