use std::ffi::CString;
use std::os::raw::{c_char, c_int};

// 3. Define a Rust struct that matches the C struct layout
#[repr(C)]
#[derive(Debug)]
struct Point {
    x: c_int,
    y: c_int,
}

// Declare external functions linked from the C library
extern "C" {
    fn c_add(a: c_int, b: c_int) -> c_int;
    fn c_print_hello(name: *const c_char);
    fn c_move_point(p: *mut Point, dx: c_int, dy: c_int);
}

fn main() {
    println!("--- Day 38: FFI with C ---");

    // 1. Calling a simple C function
    unsafe {
        let result = c_add(10, 20);
        println!("1. Called c_add(10, 20) -> {}", result);
    }

    // 2. Passing a string to C
    // We need to convert Rust String to CString (null-terminated)
    let rust_name = "Rustacean";
    let c_name = CString::new(rust_name).expect("CString::new failed");

    println!("2. Calling c_print_hello:");
    unsafe {
        c_print_hello(c_name.as_ptr());
    }

    // 3. Passing a struct pointer
    let mut p = Point { x: 1, y: 1 };
    println!("3. Point before move: {:?}", p);

    unsafe {
        c_move_point(&mut p, 5, -2);
    }

    println!("   Point after move: {:?}", p);
}
