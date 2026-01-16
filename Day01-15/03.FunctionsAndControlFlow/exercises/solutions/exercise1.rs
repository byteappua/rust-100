fn main() {
    let n1 = 10;
    let n2 = 7;

    println!("Is {} even? {}", n1, is_even(n1));
    println!("Is {} even? {}", n2, is_even(n2));
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}
