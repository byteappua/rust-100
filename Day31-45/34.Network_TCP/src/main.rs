use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => return, // Connection closed
            Ok(n) => {
                println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                // Echo back
                stream.write_all(&buffer[..n]).unwrap();
            }
            Err(_) => return,
        }
    }
}

fn main() {
    // Start Server in a separate thread
    thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        println!("Server listening on 127.0.0.1:7878");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(|| handle_client(stream));
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    });

    // Give server time to start
    thread::sleep(Duration::from_secs(1));

    // Client Code
    let mut client = TcpStream::connect("127.0.0.1:7878").unwrap();
    let msg = b"Hello, TCP!";

    client.write_all(msg).unwrap();
    println!("Sent: Hello, TCP!");

    let mut buffer = [0; 512];
    let n = client.read(&mut buffer).unwrap();
    println!("Response: {}", String::from_utf8_lossy(&buffer[..n]));
}
