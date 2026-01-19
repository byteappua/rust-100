fn main() {
    let text = "first apple banana orange";
    let converted = convert_to_pig_latin(text);
    println!("Original: {}", text);
    println!("Pig Latin: {}", converted);

    // Expected: "irst-fay apple-hay anana-bay orange-hay"
    assert_eq!(converted, "irst-fay apple-hay anana-bay orange-hay");
}

fn convert_to_pig_latin(text: &str) -> String {
    // TODO: Implement logic
    // 1. Split text into words
    // 2. Process each word
    //    - Check first char is vowel or consonant
    //    - Transform
    // 3. Join back to string
    String::new()
}
