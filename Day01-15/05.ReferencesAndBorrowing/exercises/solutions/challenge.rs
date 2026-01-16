fn main() {
    let mut s = String::new();
    create_string(&mut s);
    println!("s created: {}", s);
}

// 方式 2：修改传入的可变引用
fn create_string(buffer: &mut String) {
    buffer.push_str("hello");
}

/*
// 方式 1：直接返回 String 所有权（更常用）
fn create_string_v1() -> String {
    String::from("hello")
}
*/
