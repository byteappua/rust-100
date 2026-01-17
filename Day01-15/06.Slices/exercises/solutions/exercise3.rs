fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("Hello World");
    let word = first_word(&s);
    println!("First word: {}", word);
    assert_eq!(word, "Hello");

    let s2 = "Rust";
    let word2 = first_word(s2);
    println!("First word: {}", word2);
    assert_eq!(word2, "Rust");
}
