use daemonize::Daemonize;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "daemon" {
        return run_daemon();
    }

    println!("--- Process Management Demo (Day 83) ---");
    println!("Commands:");
    println!("  cargo run           Run IPC and Signal examples");
    println!("  cargo run -- daemon Run the daemon example");
    println!("----------------------------------------");

    // 1. IPC Example
    println!("\n[1] IPC Example: Piping data to child process");
    ipc_example()?;

    // 2. Signal Handling
    println!("\n[2] Signal Handling: Catching Ctrl+C");
    signal_example()?;

    Ok(())
}

fn ipc_example() -> io::Result<()> {
    // Example: echo "hello world" | tr a-z A-Z
    println!("    Spawning 'tr a-z A-Z' process...");

    let mut child = Command::new("tr")
        .arg("a-z")
        .arg("A-Z")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn tr command");

    // Write to stdin
    if let Some(mut stdin) = child.stdin.take() {
        println!("    Writing 'hello process world' to child stdin...");
        stdin.write_all(b"hello process world")?;
    }

    // Read from stdout
    let output = child.wait_with_output()?;

    println!("    Child exited with status: {}", output.status);
    println!(
        "    Output from child: {}",
        String::from_utf8_lossy(&output.stdout).trim()
    );

    Ok(())
}

fn signal_example() -> io::Result<()> {
    println!("    Setting up SIGINT (Ctrl+C) handler...");
    let mut signals = Signals::new(&[SIGINT])?;
    let handle = signals.handle();

    let thread_handle = thread::spawn(move || {
        for _ in signals.forever() {
            println!("\n    [Handler] Received SIGINT! (You pressed Ctrl+C)");
            break; // Exit the loop on signal
        }
    });

    println!("    I will sleep for 5 seconds. Press Ctrl+C to test the handler!");
    for i in 1..=5 {
        if thread_handle.is_finished() {
            break;
        }
        print!("    {}...", i);
        io::stdout().flush()?;
        thread::sleep(Duration::from_secs(1));
    }
    println!();

    if !thread_handle.is_finished() {
        println!("    Timeout reached (no signal). Closing handler.");
        handle.close();
    }

    thread_handle.join().expect("Thread panicked");
    println!("    Signal example finished.");

    Ok(())
}

fn run_daemon() -> io::Result<()> {
    let stdout_file = File::create("/tmp/daemon.out")?;
    let stderr_file = File::create("/tmp/daemon.err")?;

    let daemonize = Daemonize::new()
        .pid_file("/tmp/process_demo.pid")
        .working_directory("/tmp")
        .stdout(stdout_file)
        .stderr(stderr_file)
        .umask(0o027);

    println!("Attempting to daemonize...");
    println!("Check /tmp/daemon.out and /tmp/daemon.err for output.");

    match daemonize.start() {
        Ok(_) => {
            // We are now in the daemon process
            println!("Daemon started at {:?}", std::time::SystemTime::now());
            loop {
                println!("Daemon is alive...");
                thread::sleep(Duration::from_secs(5));
            }
        }
        Err(e) => eprintln!("Error starting daemon: {}", e),
    }

    Ok(())
}
