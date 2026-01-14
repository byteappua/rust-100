use std::fmt;

// 1. Newtype Pattern for Type Safety and Abstraction
struct Millimeters(u32);
#[allow(dead_code)]
struct Meters(u32);

impl fmt::Display for Millimeters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} mm", self.0)
    }
}

// 2. Type Aliases
type Kilometers = i32;

// 3. The Never Type '!'
// This function never returns.
#[allow(dead_code)]
fn bar() -> ! {
    panic!("This function never returns!");
}

fn main() {
    // Newtypes
    let mm = Millimeters(500);
    // let m = Meters(500);
    // let sum = mm + m; // Compile error: cannot add Millimeters and Meters directly
    println!("Measurement: {}", mm);

    // Type Aliases
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y); // Works because Kilometers IS i32

    // Type aliases for long types
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let _f: Thunk = Box::new(|| println!("hi"));

    // Never type usage
    let guess = "3";
    loop {
        let _num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => break, // break returns !, which can coerce to u32
        };
        // Or continue, which also returns !
        println!("Parsed number: {}", _num);
        break; // Break manually to avoid infinite loop in this demo
    }

    // DSTs (Dynamically Sized Types)
    // str is a DST. We usually use &str.
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";
    println!("{} {}", s1, s2);

    // Sized trait
    // Generic functions default to T: Sized.
    fn generic<T>(_t: T) {
        // ...
    }
    generic(5);

    // To allow DSTs, use ?Sized
    fn generic_dst<T: ?Sized>(_t: &T) {
        // ...
    }
    generic_dst("hello"); // &str -> T is str (DST)

    println!("Done demonstrating types.");

    // Uncomment to see the panic from the never type function
    // bar();
}
