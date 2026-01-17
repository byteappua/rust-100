struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn main() {
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        pages: 560,
    };

    println!("Title: {}", book.title);
    println!("Author: {}", book.author);
    println!("Pages: {}", book.pages);
}
