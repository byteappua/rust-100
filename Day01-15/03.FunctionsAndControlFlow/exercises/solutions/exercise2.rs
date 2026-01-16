fn main() {
    let n = 150;

    // if 作为表达式，必须有 else 分支，且返回类型一致
    let result = if n > 100 {
        "Big"
    } else {
        "Small"
    };

    println!("The number {} is {}", n, result);
}
