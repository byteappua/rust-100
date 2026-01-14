use std::fs::File;
use std::io::{self, Read};
use std::fs;

// Function using the ? operator
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // Simplified:
    // let mut f = File::open("hello.txt")?;
    // f.read_to_string(&mut s)?;

    // Even more simplified:
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Even simpler way using std::fs
fn read_username_simple() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    // 1. panic! example (commented out to avoid crashing the demo)
    // panic!("crash and burn");

    // 2. Handling Result
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("Created hello.txt");
                    fc
                },
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Write something to file so we can read it
    fs::write("hello.txt", "rust_user").expect("Unable to write file");

    // 3. Propagating Errors
    match read_username_from_file() {
        Ok(username) => println!("Username from file: {}", username),
        Err(e) => println!("Error reading username: {:?}", e),
    }

    match read_username_simple() {
        Ok(username) => println!("Username (simple): {}", username),
        Err(e) => println!("Error (simple): {:?}", e),
    }

    // Clean up
    fs::remove_file("hello.txt").unwrap_or_else(|e| {
        println!("Could not delete file: {}", e);
    });
}
