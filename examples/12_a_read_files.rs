use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("12_a_read_files.rs").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }
        println!("{} ({} bytes long)", line, len);

        // Shrinks the line back to 0, ...
        // ... preventing persistence into following lines
        line.truncate(0);
    }
}
