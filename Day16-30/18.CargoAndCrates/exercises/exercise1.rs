// TODO: Add documentation comments to this function.
// It should describe what the function does.
// It MUST include an "Examples" section with a code block that tests the function.

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    // You can run `rustdoc --test exercise1.rs` to verify the doc tests
    // if you have rustdoc installed, or just `cargo test` if inside a crate.
    // For this single file exercise, just writing the comments is the goal.
    println!("3 * 4 = {}", multiply(3, 4));
}
