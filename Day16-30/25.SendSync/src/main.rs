#[allow(unused_imports)]
use std::rc::Rc;
#[allow(unused_imports)]
use std::thread;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

#[allow(dead_code)]
struct MyBox<T>(T);

// Most types are Send and Sync if their contents are.
unsafe impl<T: Send> Send for MyBox<T> {}
unsafe impl<T: Sync> Sync for MyBox<T> {}

fn main() {
    // 1. Primitive types are Send + Sync
    assert_send::<i32>();
    assert_sync::<i32>();

    // 2. Rc is NEITHER Send nor Sync
    // assert_send::<Rc<i32>>(); // This would fail to compile
    // assert_sync::<Rc<i32>>(); // This would fail to compile

    println!("Assertions passed!");

    // Demonstration of why Rc is not Send:
    // If we could send Rc to another thread, both threads could clone/drop it.
    // Since Rc internal counter is not atomic, this leads to data races.

    // Uncommenting the following will cause a compilation error:
    /*
    let r = Rc::new(5);
    thread::spawn(move || {
        println!("{:?}", r);
    });
    */

    println!("Rc<T> cannot be sent between threads safely.");
}
