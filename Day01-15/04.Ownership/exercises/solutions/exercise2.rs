fn main() {
    let s = String::from("Rust");
    // 我们传递 s.clone() 给函数，保留 s 的所有权
    takes_ownership(s.clone());
    println!("I still have: {}", s);
}

fn takes_ownership(some_string: String) {
    println!("I took: {}", some_string);
}
