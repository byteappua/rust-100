fn main() {
    // 1. 作用域
    {
        let _s = "hello"; // _s 从这里开始有效
    } // _s 作用域结束

    // 2. String 与 Move
    let s1 = String::from("hello");
    let s2 = s1; // s1 被移动到 s2
    // println!("{}, world!", s1); // Error: value borrowed here after move
    println!("s2 = {}", s2);

    // 3. Clone
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // 4. Copy (Stack only)
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // x 依然有效

    // 5. 所有权与函数
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // Error: s 的所有权已经被移走

    let x = 5;
    makes_copy(x);
    println!("{}", x); // x 依然有效

    // 6. 返回值与所有权
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string 离开作用域，drop 被调用，内存释放

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer 离开作用域，无事发生

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
