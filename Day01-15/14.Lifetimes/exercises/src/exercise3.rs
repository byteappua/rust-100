// TODO: Add lifetime annotation to struct definition and field
struct Excerpt {
    part: &str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    // TODO: Initialize struct
    /*
    let i = Excerpt {
        part: first_sentence,
    };
    println!("Excerpt: {}", i.part);
    */
}
