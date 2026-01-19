trait Licensed {
    // TODO: Add default implementation
    fn licensing_info(&self) -> String {
        String::from("Default License")
    }
}

struct Car {
    model: String,
}

struct Book {
    title: String,
}

// TODO: Implement Licensed for Car (use default)

// TODO: Implement Licensed for Book (override default)

fn main() {
    let c = Car { model: String::from("Tesla") };
    let b = Book { title: String::from("Rust Book") };

    // println!("Car license: {}", c.licensing_info());
    // println!("Book license: {}", b.licensing_info());
}
