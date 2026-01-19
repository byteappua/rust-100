use std::fs::{self, File};
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Setup test files
    let file1 = "input_ex2.txt";
    let file2 = "output_ex2.txt";
    fs::write(file1, "Hello, ")?;
    fs::write(file2, "World!")?;

    // Call the function
    read_and_append(file1, file2)?;

    // Verify
    let content = fs::read_to_string(file2)?;
    println!("Final content: {}", content);
    assert_eq!(content, "World!Hello, ");

    // Cleanup
    fs::remove_file(file1)?;
    fs::remove_file(file2)?;

    Ok(())
}

fn read_and_append(source_path: &str, dest_path: &str) -> io::Result<()> {
    // TODO:
    // 1. Open source file and read content to string
    // 2. Open dest file in append mode (or read+write) and write content
    // Use `?` for error handling

    Ok(())
}
