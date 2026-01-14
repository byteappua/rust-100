/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = 6;
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// A custom struct for demonstration.
pub struct MyBox {
    /// The value inside the box
    pub value: i32,
}

fn main() {
    println!("Add one to 5: {}", add_one(5));
    println!("This project demonstrates Cargo documentation features.");
    println!("Run `cargo doc --open` to see the generated documentation.");
}
