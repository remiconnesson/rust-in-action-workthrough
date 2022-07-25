fn main() {
    // simplest code...
    let a = 10;
    let b = a + a;
    println!("a + a = {}", b);

    // ref & deref
    let a = 10;
    let r = &a;
    let b = a + *r;
    println!("a + a = {}", b);

    // implicit deref
    let a = 10;
    let r = &a;
    let b = a + r;
    println!("a + a = {}", b);
}
