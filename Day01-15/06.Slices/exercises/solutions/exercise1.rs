fn main() {
    let s = String::from("Hello World");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("First part: {}", hello);
    println!("Second part: {}", world);

    assert_eq!(hello, "Hello");
    assert_eq!(world, "World");
}
