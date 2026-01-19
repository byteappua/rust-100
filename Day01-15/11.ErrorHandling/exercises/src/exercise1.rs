fn main() {
    let result = math_operation(100.0, 4.0);
    println!("sqrt(100 / 4) = {:?}", result);
    assert_eq!(result, Some(5.0));

    let fail_div = math_operation(100.0, 0.0);
    println!("sqrt(100 / 0) = {:?}", fail_div);
    assert_eq!(fail_div, None);

    let fail_sqrt = math_operation(-100.0, 1.0);
    println!("sqrt(-100 / 1) = {:?}", fail_sqrt);
    assert_eq!(fail_sqrt, None);
}

fn checked_div(dividend: f64, divisor: f64) -> Option<f64> {
    // TODO: Return None if divisor is 0.0
    None
}

fn checked_sqrt(number: f64) -> Option<f64> {
    // TODO: Return None if number is negative
    None
}

// Calculate sqrt(x / y)
fn math_operation(x: f64, y: f64) -> Option<f64> {
    // TODO: Use checked_div then checked_sqrt.
    // You can use match, if let, or combinators like `and_then`.
    None
}
