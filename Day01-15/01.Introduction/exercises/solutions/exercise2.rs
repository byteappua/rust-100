// 练习 2: 个人信息卡片
// 难度: ⭐⭐

fn main() {
    println!("=========================");
    println!("    Personal Info Card");
    println!("=========================");
    println!("Name:       Alice");
    println!("Age:        25");
    println!("City:       Beijing");
    println!("Occupation: Software Engineer");
    println!("=========================");
}

/* 
知识点:
1. 使用多个 println! 语句输出多行
2. 使用空格进行对齐
3. ASCII 字符可以直接在字符串中使用

进阶版本 - 使用变量:
*/

#[allow(dead_code)]
fn main_with_variables() {
    let name = "Alice";
    let age = 25;
    let city = "Beijing";
    let occupation = "Software Engineer";
    
    println!("=========================");
    println!("    Personal Info Card");
    println!("=========================");
    println!("Name:       {}", name);
    println!("Age:        {}", age);
    println!("City:       {}", city);
    println!("Occupation: {}", occupation);
    println!("=========================");
}

/* 
进阶知识点:
1. 使用 let 声明变量
2. 使用 {} 在字符串中插入变量值
3. println! 支持格式化输出

运行方式:
cargo run

预期输出:
=========================
    Personal Info Card
=========================
Name:       Alice
Age:        25
City:       Beijing
Occupation: Software Engineer
=========================
*/
