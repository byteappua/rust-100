use std::num::ParseIntError;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
enum ParsePersonError {
    Format,
    BadAge(ParseIntError),
}

// Allow ParseIntError to be converted to ParsePersonError automatically when using ?
impl From<ParseIntError> for ParsePersonError {
    fn from(err: ParseIntError) -> Self {
        ParsePersonError::BadAge(err)
    }
}

fn parse_person(input: &str) -> Result<Person, ParsePersonError> {
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() != 2 {
        return Err(ParsePersonError::Format);
    }

    let name = String::from(parts[0]);
    // The ? operator works because we implemented From<ParseIntError> for ParsePersonError
    let age = parts[1].trim().parse::<u32>()?;

    Ok(Person { name, age })
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
