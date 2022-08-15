#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        // Ensure that there's sufficient space to fit the incoming data.
        save_to.reserve(read_length);
        // Allocates sufficient data in the save_to buffer to hold the contents of f.
        save_to.append(&mut tmp);
        read_length
    }
}

fn open(f: &mut File) -> bool {
    true // dummies
}

fn close(f: &mut File) -> bool {
    true // dummies
}

fn main() {
    let f2_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f2 = File::new_with_data("2.txt", &f2_data);

    let mut buffer: Vec<u8> = vec![];
    // does the hard work of interacting with the file
    open(&mut f2);
    let f2_length = f2.read(&mut buffer);
    close(&mut f2);

    // converts Vec<u8> to String.
    // Any bytes that are not valid utf8 are replaced with an invalid character.
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text); // Prints 114, 117, 115, 116, 33 as utf8(lossy).
}
