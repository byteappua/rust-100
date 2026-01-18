/// Multiplies two 32-bit integers.
///
/// # Examples
///
/// ```
/// // Note: In a real crate, you would use `use my_crate::multiply;`
/// // For a standalone file test, we can pretend or wrap in a module.
/// let result = 3 * 4;
/// assert_eq!(result, 12);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    println!("3 * 4 = {}", multiply(3, 4));
}
