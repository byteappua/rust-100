use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::mem;

fn main() {
    println!("=== OS Concepts Demo ===\n");

    // 1. Environment Variables
    demonstrate_env_vars();

    // 2. Process Management
    demonstrate_process();

    // 3. Threading
    demonstrate_threading();

    // 4. Memory Concepts (Basic)
    demonstrate_memory();

    println!("\n=== Demo Completed ===");
}

fn demonstrate_env_vars() {
    println!("--- Environment Variables ---");
    // Get a specific environment variable
    match env::var("PATH") {
        Ok(path) => println!("PATH: {:.50}...", path), // Truncate for readability
        Err(e) => println!("Could not read PATH: {}", e),
    }

    // List all environment variables (limit to first 3 for brevity)
    println!("First 3 env vars:");
    for (key, value) in env::vars().take(3) {
        println!("{}: {:.20}...", key, value);
    }
    println!();
}

fn demonstrate_process() {
    println!("--- Process Execution ---");
    // Execute a system command
    // Using 'ls' on Unix/Linux/macOS, or 'cmd /C dir' on Windows

    let mut command;
    if cfg!(target_os = "windows") {
        command = Command::new("cmd");
        command.args(["/C", "dir"]);
    } else {
        command = Command::new("ls");
        command.arg("-F"); // Classify output (append / to directories)
    }

    let output = command
        .output()
        .expect("Failed to execute command");

    println!("Command executed with status: {}", output.status);
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Stdout (first 5 lines):");
        for line in stdout.lines().take(5) {
            println!("  {}", line);
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Stderr: {}", stderr);
    }
    println!();
}

fn demonstrate_threading() {
    println!("--- Threading ---");
    let handle = thread::spawn(|| {
        println!("  [Child Thread] Started work...");
        thread::sleep(Duration::from_millis(100));
        println!("  [Child Thread] Finished work.");
        42
    });

    println!("  [Main Thread] Waiting for child thread...");
    match handle.join() {
        Ok(result) => println!("  [Main Thread] Child thread returned: {}", result),
        Err(_) => println!("  [Main Thread] Child thread panicked"),
    }
    println!();
}

fn demonstrate_memory() {
    println!("--- Memory Concepts ---");
    println!("Size of types (in bytes):");
    println!("  i32: {}", mem::size_of::<i32>());
    println!("  f64: {}", mem::size_of::<f64>());
    println!("  usize: {}", mem::size_of::<usize>()); // Architecture dependent
    println!("  Option<i32>: {}", mem::size_of::<Option<i32>>());
    println!();
}
