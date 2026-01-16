use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use memmap2::MmapOptions;
use fs2::FileExt;

fn main() -> io::Result<()> {
    println!("--- File System Demo ---");

    // 1. Recursive Directory Traversal
    println!("\n1. Walking directory './src':");
    walk_dir(Path::new("./src"))?;

    // Prepare a file for other operations
    let test_file = "test_data.txt";
    {
        let mut f = File::create(test_file)?;
        f.write_all(b"Hello, Memory Mapped World! This is some test data for locking.")?;
    } // File is closed here

    // 2. Memory Mapped File
    println!("\n2. Memory mapping file '{}':", test_file);
    mmap_file(Path::new(test_file))?;

    // 3. File Locking
    println!("\n3. Testing file locking on '{}':", test_file);
    file_lock_example(test_file)?;

    // Cleanup
    fs::remove_file(test_file)?;
    println!("\nDone.");

    Ok(())
}

// 递归遍历目录
fn walk_dir(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                walk_dir(&path)?;
            } else {
                println!("  File: {:?}", path);
                // Print some metadata
                let metadata = fs::metadata(&path)?;
                println!("    Size: {} bytes, Read-only: {}", metadata.len(), metadata.permissions().readonly());
            }
        }
    }
    Ok(())
}

fn mmap_file(path: &Path) -> io::Result<()> {
    let file = File::open(path)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    // Check length
    println!("  Mapped {} bytes", mmap.len());

    // Print first 25 bytes as string
    if mmap.len() >= 25 {
        let data = &mmap[0..25];
        println!("  First 25 bytes content: {:?}", String::from_utf8_lossy(data));
    } else {
        println!("  Content: {:?}", String::from_utf8_lossy(&mmap));
    }

    Ok(())
}

fn file_lock_example(path: &str) -> io::Result<()> {
    let file = File::open(path)?;

    println!("  Acquiring exclusive lock...");
    // 独占锁
    file.lock_exclusive()?;
    println!("  Lock acquired. Doing critical section work...");

    // Simulate work
    std::thread::sleep(std::time::Duration::from_millis(100));

    file.unlock()?;
    println!("  Lock released.");

    Ok(())
}
