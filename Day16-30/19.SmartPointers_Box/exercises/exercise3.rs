use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// TODO: Implement Deref trait for MyBox so we can use * operator.

/*
impl<T> Deref for MyBox<T> {
    // ...
}
*/

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y); // This should work after implementing Deref
}
