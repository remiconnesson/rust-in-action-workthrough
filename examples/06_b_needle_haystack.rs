fn main() {
    let haystack = [1, 1, 2, 3, 42, 46, 132, 12345];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit",
            _ => "miss",
        };

        if result == "hit" {
            println!("{}: {}", item, result);
        }
    }
}
