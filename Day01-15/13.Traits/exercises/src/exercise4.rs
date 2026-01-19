use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

// TODO: Implement fmt::Display for Point
// impl fmt::Display for Point { ... }

fn main() {
    let p = Point { x: 10, y: 20 };
    // println!("{}", p);
    // Expected output: (10, 20)
}
