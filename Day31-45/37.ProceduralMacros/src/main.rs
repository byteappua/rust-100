use hello_macro_derive::HelloMacro;

// Ideally, this trait would be defined in a separate crate (e.g., `hello_macro`),
// but for simplicity in this demo, we define it here.
pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Rustacean;

fn main() {
    println!("--- Day 37: Procedural Macros ---");
    Pancakes::hello_macro();
    Rustacean::hello_macro();
}
