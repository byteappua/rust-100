use std::fmt::Display;

fn compare_prints<T: Display + PartialOrd>(a: &T, b: &T) {
    if a > b {
        println!("{} is bigger", a);
    } else {
        println!("{} is bigger or equal", b);
    }
}

fn main() {
    let a = 10;
    let b = 20;

    compare_prints(&a, &b);

    let s1 = "Apple";
    let s2 = "Banana";
    compare_prints(&s1, &s2);
}
