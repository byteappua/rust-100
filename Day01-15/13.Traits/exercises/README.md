# Day 13 练习题

## 练习 1: 定义与实现

文件: `src/exercise1.rs`

1. 定义一个 trait `AppendBar`，包含一个方法 `append_bar(self) -> Self`。
2. 为 `String` 实现 `AppendBar`，在字符串末尾追加 "Bar"。
3. 为 `Vec<String>` 实现 `AppendBar`，向向量中 push 一个 "Bar" 字符串。

## 练习 2: Trait Bound

文件: `src/exercise2.rs`

编写一个泛型函数 `compare_prints`，它接受两个引用。这两个参数必须都实现了 `Display` 和 `PartialOrd` trait。
函数应该打印较大的那个值。
提示：使用 `use std::fmt::Display;`。

## 练习 3: 默认实现

文件: `src/exercise3.rs`

定义 `Licensed` trait。
- 方法 `licensing_info(&self) -> String`: 提供默认实现返回 "Some default license"。
- 结构体 `Car` 和 `Book`。
- 为 `Car` 实现 `Licensed` 并使用默认实现。
- 为 `Book` 实现 `Licensed` 并重写方法返回 "Copyright 2023"。

## 练习 4: 实现 Display Trait

文件: `src/exercise4.rs`

定义一个结构体 `Point { x: i32, y: i32 }`。
为它实现 `std::fmt::Display` trait，使其打印格式为 `(x, y)`。
然后在 `main` 中使用 `println!("{}", p)` 打印它。

## ✅ 验证你的答案

```bash
rustc src/exercise1.rs && ./exercise1
rustc src/exercise2.rs && ./exercise2
rustc src/exercise3.rs && ./exercise3
rustc src/exercise4.rs && ./exercise4
```
