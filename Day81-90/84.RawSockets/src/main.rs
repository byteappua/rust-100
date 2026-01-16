use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use socket2::{Socket, Domain, Type, Protocol};

fn main() -> io::Result<()> {
    println!("--- Raw Sockets & Low-level Networking Demo (Day 84) ---");

    // 1. Socket2 Creation Demo
    println!("\n[1] Creating a socket using 'socket2' crate...");
    let _socket = create_socket2_socket()?;
    println!("    Successfully created a configured socket (IPV4, STREAM, TCP).");
    println!("    Socket is non-blocking: true");
    println!("    Socket reuse address: true");
    // We won't do much with this raw socket in this demo to avoid complexity/permissions,
    // but this demonstrates how to configure it before binding.

    // 2. Simple TCP Echo Server
    println!("\n[2] Starting a simple TCP Echo Server on 127.0.0.1:8084...");

    // Spawn server in a separate thread
    thread::spawn(|| {
        if let Err(e) = run_tcp_server() {
            eprintln!("Server error: {}", e);
        }
    });

    // Give server a moment to start
    thread::sleep(Duration::from_millis(500));

    // 3. Simple TCP Client
    println!("\n[3] Connecting client to 127.0.0.1:8084...");
    run_tcp_client()?;

    // Wait for server thread (in a real app we'd need a way to signal shutdown)
    // For this demo, we'll just let the main process exit, which kills threads.
    println!("\nDemo completed.");
    Ok(())
}

fn create_socket2_socket() -> io::Result<Socket> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;

    // Set some options that might not be available on standard std::net::TcpStream builder
    socket.set_nonblocking(true)?;
    socket.set_reuse_address(true)?;

    // On some platforms, we could set more specific options here (e.g., TOS, TTL)

    Ok(socket)
}

fn run_tcp_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8084")?;
    println!("    [Server] Listening on port 8084...");

    // Accept one connection for the demo, then exit loop to finish
    // In a real server, this would be `for stream in listener.incoming()`
    if let Some(stream) = listener.incoming().next() {
        match stream {
            Ok(stream) => {
                println!("    [Server] New connection: {:?}", stream.peer_addr()?);
                handle_client(stream)?;
            }
            Err(e) => eprintln!("    [Server] Connection failed: {}", e),
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];

    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            break; // Connection closed
        }

        let received = String::from_utf8_lossy(&buffer[0..n]);
        println!("    [Server] Received: {:?}", received);

        stream.write_all(&buffer[0..n])?;
        println!("    [Server] Echoed back {} bytes", n);
    }
    Ok(())
}

fn run_tcp_client() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8084")?;
    println!("    [Client] Connected.");

    let msg = "Hello from Day 84!";
    stream.write_all(msg.as_bytes())?;
    println!("    [Client] Sent: {:?}", msg);

    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[0..n]);
    println!("    [Client] Received echo: {:?}", response);

    Ok(())
}
