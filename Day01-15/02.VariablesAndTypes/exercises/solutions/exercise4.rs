fn main() {
    let person = ("Alice", 30, 1.75);

    // Debug 打印元组
    println!("Tuple: {:?}", person);

    // 解构
    let (name, age, height) = person;

    println!("Name: {}, Age: {}, Height: {}", name, age, height);
}
