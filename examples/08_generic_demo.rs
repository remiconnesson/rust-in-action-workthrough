use std::ops::Add;
use std::time::Duration;

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    let durations = add(Duration::new(5, 0), Duration::new(10, 0));

    println!("{}", floats);
    println!("{}", ints); // {} requests std::fmt::Display
    println!("{:?}", durations); // {:?} requests std::fmt::Debug
}
