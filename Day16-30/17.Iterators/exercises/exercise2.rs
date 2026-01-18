struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// TODO: Implement the Iterator trait for Counter.
// It should count from 1 to 5, then return None.

/*
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}
*/

fn main() {
    let mut counter = Counter::new();

    // println!("{:?}", counter.next()); // Some(1)
    // println!("{:?}", counter.next()); // Some(2)
    // ...
    // println!("{:?}", counter.next()); // None
}
