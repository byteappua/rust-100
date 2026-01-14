fn main() {
    // 1. 字符串切片
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    // 2. 字符串字面值作为参数
    let my_string = String::from("hello world");
    // first_word 接受 &String 的切片
    let word = first_word(&my_string[..]);
    println!("First word: {}", word);

    let my_string_literal = "hello world";
    // first_word 接受字符串字面值切片
    let word = first_word(&my_string_literal[..]);
    println!("First word: {}", word);

    // 因为字符串字面值本来就是切片
    let word = first_word(my_string_literal);
    println!("First word: {}", word);

    // 3. 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("Slice: {:?}", slice);
}

// 练习：获取第一个单词
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
