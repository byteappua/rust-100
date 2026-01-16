fn main() {
    let s1 = String::from("hello");
    // 使用 .clone() 创建深拷贝
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
