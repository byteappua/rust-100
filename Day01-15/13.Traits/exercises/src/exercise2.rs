use std::fmt::Display;

// TODO: Complete the function signature to accept T that implements Display and PartialOrd
fn compare_prints<T>(a: &T, b: &T) {
    if a > b {
        println!("{} is bigger", a);
    } else {
        println!("{} is bigger or equal", b);
    }
}

fn main() {
    let a = 10;
    let b = 20;

    // This call fails until you add the Trait Bounds
    compare_prints(&a, &b);

    let s1 = "Apple";
    let s2 = "Banana";
    compare_prints(&s1, &s2);
}
