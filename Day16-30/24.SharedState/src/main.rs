use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Mutex provides interior mutability, similar to RefCell, but thread-safe.
    // Arc is Atomic Reference Counting, thread-safe version of Rc.

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
