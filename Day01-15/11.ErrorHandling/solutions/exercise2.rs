use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Setup test files
    let file1 = "input_sol2.txt";
    let file2 = "output_sol2.txt";
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
    let mut source_content = String::new();
    fs::File::open(source_path)?.read_to_string(&mut source_content)?;

    let mut dest_file = OpenOptions::new()
        .append(true)
        .open(dest_path)?;

    dest_file.write_all(source_content.as_bytes())?;

    Ok(())
}
