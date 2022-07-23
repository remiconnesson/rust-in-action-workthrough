#![allow(dead_code)]
#[derive(Debug)]
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    // drop(grains); // this would create a dangling pointer if not caught
    println!("{:?}", grains);
}
