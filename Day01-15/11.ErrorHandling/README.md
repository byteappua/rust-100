# Day 11: 错误处理 (Error Handling)

## 📝 学习目标
- 理解 Rust 错误处理哲学：可恢复 vs 不可恢复错误
- 掌握 `Result<T, E>` 枚举及其常用方法 (`unwrap`, `expect`)
- 熟练使用 `?` 运算符传播错误
- 了解如何在 `main` 函数中处理错误
- 掌握 `panic!` 的使用场景

## 🎯 为什么要学这个
Rust 以安全著称，而处理错误是安全的重要组成部分。
- **强制处理**: Rust 不使用异常 (Exception)。它迫使你在编译时就必须决定如何处理可能发生的错误，从而消除了运行时未捕获异常导致的崩溃。
- **清晰明确**: 函数签名明确指出了可能失败的操作。

## 📖 核心概念

### 1. 错误的分类
- **不可恢复错误**: 严重的 bug，如数组越界。Rust 使用 `panic!` 宏来处理，程序会打印错误信息并退出。
- **可恢复错误**: 预期的失败，如文件未找到。Rust 使用 `Result<T, E>` 类型来处理，允许程序做出响应（如创建新文件）。

### 2. Result 枚举
```rust
enum Result<T, E> {
    Ok(T),  // 操作成功，包含结果值
    Err(E), // 操作失败，包含错误信息
}
```

### 3. 处理 Result
- **match**: 最基本的处理方式，显式处理每种情况。
- **unwrap()**: 如果是 `Ok` 返回值，如果是 `Err` 直接 panic。用于原型开发或确定不会失败的场景。
- **expect(msg)**: 同 `unwrap`，但可以指定 panic 时的错误信息（推荐）。

### 4. 传播错误 (?)
当函数内部发生错误，且你希望由调用者来处理时，可以使用 `?` 运算符。

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?; // 失败则直接返回 Err
    Ok(s)
}
```

## 💻 代码示例

### 示例 1: 基本的 Result 处理
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => {
                panic!("There was a problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

### 示例 2: 使用 unwrap_or_else
这是处理错误的更 Rust 风格的方式，避免了深层嵌套的 match。

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

## 🏋️ 练习题

我们准备了练习题来帮助你掌握错误处理模式。

- **练习 1**: 安全的数学运算
- **练习 2**: 文件读取与错误传播
- **练习 3**: 解析 CSV 数据

👉 **[点击这里查看练习题](./exercises/README.md)**

## 🤔 常见问题 (FAQ)

### Q1: 什么时候用 panic! 什么时候用 Result？
A:
- 如果错误是由于调用者使用了错误的代码（Bug）导致的（如索引越界），或者处于无法恢复的损坏状态，使用 `panic!`。
- 如果错误是预期的（如网络超时、文件不存在），使用 `Result`。
- 在示例代码、原型代码和测试中，常用 `unwrap` (即 panic)。

### Q2: 可以在 main 函数中使用 ? 吗？
A: 可以，但 `main` 函数必须返回 `Result`。
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
```

## 💡 最佳实践
- **不要滥用 unwrap**: 在生产代码中，尽量使用 `match`, `unwrap_or`, `expect` 或 `?`，除非你 100% 确定不会失败。
- **自定义错误**: 对于库的开发者，定义自己的错误类型（通常是一个 Enum），可以更清晰地表达错误意图。

## 🔗 扩展阅读
- [Rust 程序设计语言 - 错误处理](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

## ⏭️ 下一步
有了错误处理的基础，我们可以编写更健壮的程序。接下来我们将学习如何编写更通用、更灵活的代码。

下一节: [Day 12: 泛型](../12.Generics/README.md)
