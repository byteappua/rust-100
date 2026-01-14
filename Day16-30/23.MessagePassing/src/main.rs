use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 1. Simple Message Passing
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // Error: value borrowed here after move
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    println!("-------------------");

    // 2. Sending Multiple Values & Cloning the Transmitter
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Treating rx as an iterator
    for received in rx {
        println!("Got: {}", received);
    }
}
