fn main() {
    println!("Sum 1 to 5 (for loop) = {}", sum_for(5));
    println!("Sum 1 to 100 (while loop) = {}", sum_while(100));
}

fn sum_for(n: i32) -> i32 {
    let mut sum = 0;
    // 1..=n 表示包含 n 的范围
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn sum_while(n: i32) -> i32 {
    let mut sum = 0;
    let mut i = 1;
    while i <= n {
        sum += i;
        i += 1;
    }
    sum
}
