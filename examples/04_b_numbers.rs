fn main() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22_i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{} {} {} {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [42.0, 42f32, 42_f32];
    for forty_two in forty_twos {
        println!("{:02}", forty_two);
    }

    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("Base 10: {} {} {}", three, thirty, three_hundred);
    println!("Base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("Base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("Base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
