fn main() {
    let x = "5";
    println!("Step 1: x is string \"{}\"", x);

    let x: i32 = x.parse().expect("Not a number");
    println!("Step 2: x is integer {}", x);

    let x = x + 1;
    println!("Step 3: x + 1 = {}", x);

    let x = x * 3;
    println!("Step 4: x * 3 = {}", x);
}
