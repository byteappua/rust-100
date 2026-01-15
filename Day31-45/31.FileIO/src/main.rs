use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("hello.txt");

    // 1. Write to a file
    let mut file = File::create(path)?;
    file.write_all(b"Hello, world!\n")?;
    file.write_all(b"This is the second line.\n")?;
    println!("Successfully wrote to {:?}", path);

    // 2. Read from a file (Read entire string)
    let content = fs::read_to_string(path)?;
    println!("--- File Content (read_to_string) ---");
    println!("{}", content);

    // 3. Append to a file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)?;
    writeln!(file, "This is an appended line.")?;
    println!("Successfully appended to {:?}", path);

    // 4. Read line by line using BufReader
    println!("--- File Content (BufReader) ---");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?; // unwrap Result
        println!("{}: {}", index + 1, line);
    }

    // 5. Cleanup
    fs::remove_file(path)?;
    println!("Successfully removed {:?}", path);

    Ok(())
}
