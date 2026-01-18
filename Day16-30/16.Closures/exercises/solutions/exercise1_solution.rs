fn main() {
    let input = "rust";

    // 闭包接受 &str 参数，使用 to_uppercase() 方法
    let to_uppercase = |s: &str| s.to_uppercase();

    println!("Input: {}, Output: {}", input, to_uppercase(input));
}
