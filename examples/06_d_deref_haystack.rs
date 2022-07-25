fn main() {
    let needle = 0o204;
    let haystack = [1, 2, 132, 12345];

    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}
