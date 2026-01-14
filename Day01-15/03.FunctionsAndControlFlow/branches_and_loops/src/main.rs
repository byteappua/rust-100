fn main() {
    // 1. 函数调用
    another_function(5);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    // 2. Control Flow - if
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // 3. Control Flow - loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    // 4. Control Flow - while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 5. Control Flow - for
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // 练习：摄氏度转华氏度
    let c = 32.0;
    let f = celsius_to_fahrenheit(c);
    println!("{:.1} C = {:.1} F", c, f);

    // 练习：斐波那契
    let n = 10;
    println!("Fibonacci({}) = {}", n, fibonacci(n));
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
