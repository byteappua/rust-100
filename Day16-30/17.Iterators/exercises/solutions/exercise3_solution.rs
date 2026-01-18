fn main() {
    let words = vec!["hi", "hello", "rust", "is", "awesome"];

    let result: String = words.iter()
        .filter(|w| w.len() > 3)
        .map(|w| w.to_uppercase())
        .collect::<Vec<String>>() // First collect to Vec<String>
        .join(",");               // Then join

    // Note: A more efficient way without intermediate Vec allocation
    // involves using fold or itertools, but this is standard.

    println!("{}", result);
    assert_eq!(result, "HELLO,RUST,AWESOME");
}
