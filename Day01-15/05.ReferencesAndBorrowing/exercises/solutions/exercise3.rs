fn main() {
    let mut s = String::from("Rust");
    add_brackets(&mut s);
    println!("{}", s);
}

fn add_brackets(s: &mut String) {
    s.insert_str(0, "[");
    s.push_str("]");
}
