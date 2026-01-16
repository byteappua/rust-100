fn main() {
    let mut x = 10;
    increment(&mut x);
    println!("x is now {}", x);
}

fn increment(n: &mut i32) {
    *n += 1; // 使用 * 解引用来修改值
}
