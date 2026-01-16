// 练习 3: 简单计算器
// 难度: ⭐⭐⭐

fn main() {
    println!("Simple Calculator");
    println!("=================");
    
    // 加法
    let a = 10;
    let b = 20;
    let sum = a + b;
    println!("{} + {} = {}", a, b, sum);
    
    // 减法
    let c = 50;
    let d = 15;
    let difference = c - d;
    println!("{} - {} = {}", c, d, difference);
    
    // 乘法
    let e = 6;
    let f = 7;
    let product = e * f;
    println!("{} * {} = {}", e, f, product);
    
    // 除法
    let g = 100;
    let h = 4;
    let quotient = g / h;
    println!("{} / {} = {}", g, h, quotient);
    
    // 取模
    let i = 17;
    let j = 5;
    let remainder = i % j;
    println!("{} % {} = {}", i, j, remainder);
}

/* 
知识点:
1. 使用 let 声明变量
2. Rust 的基本算术运算符: +, -, *, /, %
3. println! 可以使用多个 {} 占位符
4. 变量默认是不可变的(immutable)

进阶版本 - 使用函数:
*/

#[allow(dead_code)]
fn calculate(a: i32, b: i32, op: char) -> i32 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        '%' => a % b,
        _ => 0,
    }
}

#[allow(dead_code)]
fn main_with_function() {
    println!("Simple Calculator");
    println!("=================");
    
    println!("10 + 20 = {}", calculate(10, 20, '+'));
    println!("50 - 15 = {}", calculate(50, 15, '-'));
    println!("6 * 7 = {}", calculate(6, 7, '*'));
    println!("100 / 4 = {}", calculate(100, 4, '/'));
    println!("17 % 5 = {}", calculate(17, 5, '%'));
}

/* 
进阶知识点:
1. 函数定义和参数
2. match 表达式(模式匹配)
3. 函数返回值

运行方式:
cargo run

预期输出:
Simple Calculator
=================
10 + 20 = 30
50 - 15 = 35
6 * 7 = 42
100 / 4 = 25
17 % 5 = 2
*/
