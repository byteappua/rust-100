# Day 11 练习题

## 练习 1: 安全的数学运算

文件: `src/exercise1.rs`

编写一个除法函数 `checked_div` 和一个平方根函数 `checked_sqrt`。
- 如果除数为 0，返回 `None`。
- 如果对负数求平方根，返回 `None`。
- 使用 `match` 或 `if let` 组合这两个函数：计算 `x / y` 的平方根。

## 练习 2: 错误传播

文件: `src/exercise2.rs`

编写一个函数 `read_and_append`，它接收两个文件路径。
1. 读取第一个文件的内容。
2. 将内容追加到第二个文件中。
3. 使用 `?` 运算符处理所有 I/O 错误。

注意：为了简化练习，你可以使用 `MockFile` 结构体模拟文件操作，或者直接使用 `std::fs` 但需要确保文件存在（我们会在 `main` 中创建临时文件）。

## 练习 3: 数据解析与自定义错误

文件: `src/exercise3.rs`

给定一个字符串列表，每个字符串包含 "Name,Age"。
编写一个函数解析这些字符串为 `Person` 结构体。
- 如果格式不正确（没有逗号），返回自定义错误。
- 如果年龄不是数字，返回自定义错误。
- 使用 `Result` 收集所有解析成功的人员，或者在遇到第一个错误时返回。

## ✅ 验证你的答案

```bash
rustc src/exercise1.rs && ./exercise1
rustc src/exercise2.rs && ./exercise2
rustc src/exercise3.rs && ./exercise3
```
