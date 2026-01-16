// 练习 4: ASCII 艺术
// 难度: ⭐⭐⭐⭐

fn main() {
    // Rust Logo ASCII Art
    println!(r#"
    _____           _   
   |  __ \         | |  
   | |__) |   _ ___| |_ 
   |  _  / | | / __| __|
   | | \ \ |_| \__ \ |_ 
   |_|  \_\__,_|___/\__|
    "#);
}

/* 
知识点:
1. r#"..."# 是原始字符串字面量,不需要转义反斜杠
2. 多行字符串可以直接换行
3. ASCII 艺术需要注意对齐

更多 ASCII 艺术示例:
*/

#[allow(dead_code)]
fn rust_crab() {
    println!(r#"
        _~^~^~_
    \) /  o o  \ (/
      '_   -   _'
      / '-----' \
    "#);
    println!("    Ferris the Crab! 🦀");
}

#[allow(dead_code)]
fn rust_gear() {
    println!(r#"
       ___
      /   \
     | .-. |
      \   /
       '-'
    "#);
    println!("   Rust Gear");
}

#[allow(dead_code)]
fn welcome_banner() {
    println!(r#"
╔════════════════════════════════════════╗
║                                        ║
║     Welcome to the Rust World!         ║
║                                        ║
║     Where Safety Meets Performance     ║
║                                        ║
╚════════════════════════════════════════╝
    "#);
}

#[allow(dead_code)]
fn main_showcase() {
    println!("=== ASCII Art Showcase ===\n");
    
    println!("1. Rust Logo:");
    main();
    
    println!("\n2. Ferris the Crab:");
    rust_crab();
    
    println!("\n3. Rust Gear:");
    rust_gear();
    
    println!("\n4. Welcome Banner:");
    welcome_banner();
}

/* 
进阶知识点:
1. 原始字符串字面量 r#"..."#
2. Unicode 字符(如 ╔ ║ ╚)可以直接使用
3. 函数可以调用其他函数

提示:
- 可以使用在线工具生成 ASCII 艺术: https://patorjk.com/software/taag/
- 注意保持字符对齐
- 使用 r#"..."# 避免转义字符的麻烦

运行方式:
cargo run

预期输出:
    _____           _   
   |  __ \         | |  
   | |__) |   _ ___| |_ 
   |  _  / | | / __| __|
   | | \ \ |_| \__ \ |_ 
   |_|  \_\__,_|___/\__|
*/
