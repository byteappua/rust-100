fn main() {
    // 1. 不可变引用
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 2. 可变引用
    let mut s = String::from("hello");
    change(&mut s);
    println!("Changed string: {}", s);

    // 3. 引用规则演示
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 和 r2 的作用域在这里结束（因为后面不再使用了）

    let r3 = &mut s;
    println!("{}", r3);

    // 下面这行代码打开注释会报错，因为不能同时存在可变和不可变引用
    // println!("{}, {}, and {}", r1, r2, r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
// 悬垂引用示例 (编译不过)
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/
