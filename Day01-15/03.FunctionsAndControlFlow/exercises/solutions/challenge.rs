fn main() {
    println!("Fibonacci sequence:");
    for i in 0..11 {
        println!("Fib({}) = {}", i, fib(i));
    }
}

// 递归版本（简单但效率低）
#[allow(dead_code)]
fn fib_recursive(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fib_recursive(n - 1) + fib_recursive(n - 2)
}

// 循环版本（高效）
fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let temp = curr;
        curr = prev + curr;
        prev = temp;
    }

    curr
}
