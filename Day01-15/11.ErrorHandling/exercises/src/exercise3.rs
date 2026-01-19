use std::num::ParseIntError;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
enum ParsePersonError {
    // TODO: Define meaningful error variants
    BadFormat,
    BadAge(ParseIntError),
}

fn parse_person(input: &str) -> Result<Person, ParsePersonError> {
    // TODO: Implement parsing logic
    // For now we just return an error to let code compile
    Err(ParsePersonError::BadFormat)
}

fn main() {
    let inputs = vec![
        "Alice,30",
        "Bob,invalid",
        "Charlie", // missing comma
        "David,25",
    ];

    for input in inputs {
        match parse_person(input) {
            Ok(p) => println!("Success: {:?}", p),
            Err(e) => println!("Failed to parse '{}': {:?}", input, e),
        }
    }
}
