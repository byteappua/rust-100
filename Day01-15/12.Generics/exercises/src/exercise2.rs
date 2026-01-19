// TODO: Define generic function swap_values<T>
fn swap_values<T>(a: T, b: T) -> (T, T) {
    // Placeholder implementation
    (a, b) // This is wrong, it should be (b, a)
}

fn main() {
    let (a, b) = swap_values(10, 20);
    println!("a: {}, b: {}", a, b);
    // assert_eq!(a, 20); // This would fail with the placeholder
    // assert_eq!(b, 10);

    let (s1, s2) = swap_values("World", "Hello");
    println!("{} {}", s1, s2);
}
