# Day 14 练习题

## 练习 1: 修复生命周期问题

文件: `src/exercise1.rs`

代码试图在外部作用域使用内部作用域创建的变量的引用。
请修复它（提示：可能需要移动变量定义的位置，或者改变数据的所有权）。

## 练习 2: 带生命周期的函数

文件: `src/exercise2.rs`

完成 `longest` 函数的签名和实现。它接受两个字符串切片，返回较长的那个。
你需要添加正确的生命周期注解。

## 练习 3: 结构体中的生命周期

文件: `src/exercise3.rs`

定义一个结构体 `Excerpt`，它持有一个字符串切片字段 `part`。
在 `main` 函数中，创建一个 `String`，然后创建一个持有该 String 一部分的 `Excerpt` 实例。
确保添加必要的生命周期注解。

## ✅ 验证你的答案

```bash
rustc src/exercise1.rs && ./exercise1
rustc src/exercise2.rs && ./exercise2
rustc src/exercise3.rs && ./exercise3
```
