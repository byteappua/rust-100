fn main() {
    // 1. 变量与可变性
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // 编译错误：cannot assign twice to immutable variable `x`

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // 2. 常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    // 3. 隐藏 (Shadowing)
    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {}", z); // 12
    }
    println!("The value of z is: {}", z); // 6

    // 4. 数据类型 - 标量
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Parsed number: {}", guess);

    let f = 2.0; // f64
    let b: bool = false;
    let c = 'z';
    let heart_eyed_cat = '😻';
    println!("Float: {}, Bool: {}, Char: {}, Emoji: {}", f, b, c, heart_eyed_cat);

    // 5. 数据类型 - 复合
    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (t1, t2, _t3) = tup;
    println!("Tuple values: {}, {}", t1, t2);

    // 数组
    let a = [1, 2, 3, 4, 5];
    println!("First element of array: {}", a[0]);
}
