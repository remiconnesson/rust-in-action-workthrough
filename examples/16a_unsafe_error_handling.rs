#![allow(unused_variables)]
use rand::random;

static mut ERROR: i32 = 0;

struct File(String);

fn open(f: &mut File) {
    unsafe {
        if random() {
            ERROR = 1;
        };
    }
}

fn close(f: &mut File) {
    unsafe {
        if random() {
            ERROR = 1;
        };
    }
}

fn main() {
    let mut f = File(String::from("2.txt"));

    open(&mut f);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occured while reading the file");
        }
    }

    close(&mut f);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occured while closing the file");
        }
    }
}
