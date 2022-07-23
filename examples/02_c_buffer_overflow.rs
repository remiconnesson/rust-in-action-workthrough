fn main() {
    let fruits = vec!["banana", "apple", "tomato"];

    let buffer_overflow = fruits[4];

    assert_eq!(buffer_overflow, "watermelon");
}
