fn main() {
    let text = String::from("Rust is amazing and safe");
    let count = word_count(&text);
    println!("Word count: {}", count);
}

fn word_count(s: &String) -> usize {
    s.split_whitespace().count()
}
