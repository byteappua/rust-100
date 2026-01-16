#![allow(static_mut_refs)]
use std::slice;

// 3. Modifying a static mutable variable
// Global variables are problematic for concurrency, so accessing mutable statics is unsafe.
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// 2. Calling an unsafe function / Creating a safe abstraction
// This function mimics std::slice::split_at_mut but simplified
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// 4. Calling External Code (FFI)
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    println!("--- Day 36: Unsafe Rust ---");

    // 1. Dereferencing Raw Pointers
    println!("\n1. Raw Pointers:");
    let mut num = 5;

    // Creating raw pointers is safe
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        // Dereferencing is unsafe
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        *r2 = 10;
        println!("r2 changed to: {}", *r2);
    }
    println!("num is now: {}", num);

    // 2. Safe Abstraction over Unsafe Code
    println!("\n2. Safe Abstraction (split_at_mut):");
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    println!("Part A: {:?}", a);
    println!("Part B: {:?}", b);

    // Mutating safe slices that originated from unsafe pointer arithmetic
    a[0] = 100;
    b[0] = 200;
    println!("Modified Vector: {:?}", v);


    // 3. Static Mut
    println!("\n3. Mutable Static Variables:");
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // 4. FFI
    println!("\n4. FFI (Calling C 'abs'):");
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
