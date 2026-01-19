fn main() {
    let text = "first apple banana orange";
    let converted = convert_to_pig_latin(text);
    println!("Original: {}", text);
    println!("Pig Latin: {}", converted);

    assert_eq!(converted, "irst-fay apple-hay anana-bay orange-hay");
}

fn convert_to_pig_latin(text: &str) -> String {
    let mut results = Vec::new();

    for word in text.split_whitespace() {
        let first_char = word.chars().next().unwrap();
        let is_vowel = match first_char.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        };

        if is_vowel {
            results.push(format!("{}-hay", word));
        } else {
            // Be careful with UTF-8 chars, but for this exercise we assume ascii-like behavior for "first char"
            // actually string slicing like `&word[1..]` might panic if first char is multi-byte.
            // Safe way: collect chars.
            let mut chars = word.chars();
            chars.next(); // skip first
            let rest: String = chars.collect();
            results.push(format!("{}-{}ay", rest, first_char));
        }
    }

    results.join(" ")
}
