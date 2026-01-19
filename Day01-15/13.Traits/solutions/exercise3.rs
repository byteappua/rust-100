trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Default License")
    }
}

struct Car {
    #[allow(dead_code)]
    model: String,
}

struct Book {
    #[allow(dead_code)]
    title: String,
}

impl Licensed for Car {}

impl Licensed for Book {
    fn licensing_info(&self) -> String {
        String::from("Copyright 2023")
    }
}

fn main() {
    let c = Car { model: String::from("Tesla") };
    let b = Book { title: String::from("Rust Book") };

    println!("Car license: {}", c.licensing_info());
    assert_eq!(c.licensing_info(), "Default License");

    println!("Book license: {}", b.licensing_info());
    assert_eq!(b.licensing_info(), "Copyright 2023");
}
