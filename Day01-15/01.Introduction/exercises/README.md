# Day 01 练习题 - 初识 Rust

## 🎯 练习目标

通过这些练习,你将:
- 熟悉 Rust 的基本语法
- 掌握 Cargo 的使用
- 学会编写简单的 Rust 程序

## 📝 练习列表

### 练习 1: Hello, Rust! (基础)
**难度**: ⭐

**题目**: 
创建一个 Rust 程序,输出 "Hello, Rust! Welcome to the world of systems programming!"

**要求**:
- 使用 `cargo new` 创建项目
- 在 `main.rs` 中实现
- 使用 `cargo run` 运行

**提示**:
- 使用 `println!` 宏输出文本
- 注意 Rust 的字符串使用双引号

**参考答案**: [查看答案](./solutions/exercise1.rs)

---

### 练习 2: 个人信息卡片 (基础)
**难度**: ⭐⭐

**题目**:
编写一个程序,输出你的个人信息卡片,包括姓名、年龄、城市和职业。

**示例输出**:
```
=========================
    Personal Info Card
=========================
Name:       Alice
Age:        25
City:       Beijing
Occupation: Software Engineer
=========================
```

**要求**:
- 使用多个 `println!` 语句
- 注意格式对齐
- 使用 ASCII 字符绘制边框

**提示**:
- 可以使用 `println!` 的格式化功能
- 使用 `=` 和空格来对齐

**参考答案**: [查看答案](./solutions/exercise2.rs)

---

### 练习 3: 简单计算器 (进阶)
**难度**: ⭐⭐⭐

**题目**:
创建一个简单的计算器程序,计算并输出以下表达式的结果:
- 10 + 20
- 50 - 15
- 6 * 7
- 100 / 4
- 17 % 5

**示例输出**:
```
Simple Calculator
=================
10 + 20 = 30
50 - 15 = 35
6 * 7 = 42
100 / 4 = 25
17 % 5 = 2
```

**要求**:
- 使用变量存储数字
- 使用 Rust 的算术运算符
- 格式化输出结果

**提示**:
- 使用 `let` 声明变量
- 使用 `{}` 在 `println!` 中插入变量值

**参考答案**: [查看答案](./solutions/exercise3.rs)

---

### 练习 4: ASCII 艺术 (挑战)
**难度**: ⭐⭐⭐⭐

**题目**:
使用 ASCII 字符绘制一个 Rust 的 logo 或者你喜欢的图案。

**示例输出**:
```
    _____           _   
   |  __ \         | |  
   | |__) |   _ ___| |_ 
   |  _  / | | / __| __|
   | | \ \ |_| \__ \ |_ 
   |_|  \_\__,_|___/\__|
```

**要求**:
- 至少 5 行高度
- 使用多种 ASCII 字符
- 保持良好的对齐

**提示**:
- 使用 `\` 需要转义为 `\\`
- 可以使用在线 ASCII 艺术生成器辅助
- 注意字符串中的空格

**参考答案**: [查看答案](./solutions/exercise4.rs)

---

### 练习 5: Cargo 项目结构 (实践)
**难度**: ⭐⭐

**题目**:
创建一个完整的 Cargo 项目,包含:
1. 自定义的项目名称
2. 修改 `Cargo.toml` 添加作者信息
3. 在 `main.rs` 中输出项目信息

**要求**:
- 使用 `cargo new my_first_project` 创建项目
- 在 `Cargo.toml` 中添加 `authors` 字段
- 程序输出项目名称和作者信息

**提示**:
- `Cargo.toml` 是 TOML 格式的配置文件
- 可以使用 `env!("CARGO_PKG_NAME")` 获取包名

**参考答案**: [查看答案](./solutions/exercise5/)

---

## 🏆 挑战任务

完成以上所有练习后,尝试这个综合挑战:

### 综合挑战: 欢迎程序
创建一个欢迎程序,包含:
1. ASCII 艺术的标题
2. 欢迎信息
3. 简单的数学计算展示
4. 项目信息输出

**示例输出**:
```
╔════════════════════════════════╗
║   Welcome to Rust World!       ║
╚════════════════════════════════╝

Hello, Future Rustacean!

Let me show you some calculations:
  2 + 2 = 4
  10 * 5 = 50
  100 / 4 = 25

Project: my_first_project
Author: Your Name
Version: 0.1.0

Happy coding! 🦀
```

## 📚 学习资源

- [The Rust Programming Language - Chapter 1](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust by Example - Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)

## 💡 提示

1. **遇到编译错误?** 仔细阅读错误信息,Rust 的编译器会给出很有帮助的提示
2. **不知道如何开始?** 先看参考答案,理解后自己重新实现
3. **想要更多挑战?** 尝试组合多个练习,创建更复杂的程序

## ✅ 完成检查清单

- [ ] 完成练习 1
- [ ] 完成练习 2
- [ ] 完成练习 3
- [ ] 完成练习 4
- [ ] 完成练习 5
- [ ] 完成综合挑战
- [ ] 理解所有代码的含义
- [ ] 能够独立编写类似程序

## 🎓 下一步

完成这些练习后,你应该:
- 熟悉 Rust 的基本语法
- 掌握 Cargo 的基本使用
- 能够编写简单的 Rust 程序

准备好了吗?让我们继续 [Day 02: 变量与数据类型](../../02.VariablesAndTypes/README.md)!

---

**创建日期**: 2026-01-16
**最后更新**: 2026-01-16
