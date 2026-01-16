fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // 在这里使用 r1 和 r2
    println!("r1: {}, r2: {}", r1, r2);
    // r1 和 r2 的作用域在这里实际上已经结束了（最后一次使用）

    // 所以这里可以安全地创建可变引用
    let r3 = &mut s;
    r3.push_str(", world");
    println!("r3: {}", r3);
}
