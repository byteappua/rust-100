fn swap_values<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

fn main() {
    let (a, b) = swap_values(10, 20);
    println!("a: {}, b: {}", a, b);
    assert_eq!(a, 20);
    assert_eq!(b, 10);

    let (s1, s2) = swap_values("World", "Hello");
    println!("{} {}", s1, s2);
    assert_eq!(s1, "Hello");
}
