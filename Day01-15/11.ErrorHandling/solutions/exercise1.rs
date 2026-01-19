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
    if divisor == 0.0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn checked_sqrt(number: f64) -> Option<f64> {
    if number < 0.0 {
        None
    } else {
        Some(number.sqrt())
    }
}

fn math_operation(x: f64, y: f64) -> Option<f64> {
    // Using `and_then` combinator
    checked_div(x, y).and_then(checked_sqrt)

    // Alternative using match:
    /*
    match checked_div(x, y) {
        Some(d) => checked_sqrt(d),
        None => None,
    }
    */
    // Alternative using ? (in a function returning Option):
    /*
    let d = checked_div(x, y)?;
    checked_sqrt(d)
    */
}
