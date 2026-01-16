# Day 11: 错误处理 (Error Handling)

Rust 的错误处理机制设计得非常安全，它迫使你在编写代码时就必须处理可能发生的错误。

## 1. 错误分类

1.  **可恢复错误 (Recoverable errors)**:
    *   例如：文件未找到。
    *   处理方式：向用户报告错误，重试操作。
    *   类型：`Result<T, E>`。
2.  **不可恢复错误 (Unrecoverable errors)**:
    *   例如：数组越界访问。
    *   这是 Bug 的症状。
    *   处理方式：立即停止程序 (`panic!`)。

## 2. panic! 与不可恢复错误

```rust
fn main() {
    panic!("crash and burn");
}
```

*   默认情况下，`panic!` 会打印错误信息，展开 (unwind) 并清理栈，然后退出。
*   **Backtrace**: 设置环境变量 `RUST_BACKTRACE=1` 可以查看 panic 发生时的调用栈。

## 3. Result 与可恢复错误

`Result` 枚举包含两个成员：`Ok` 表示成功，`Err` 表示失败。

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 使用 match 处理 Result

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };
}
```

### 匹配不同的错误

```rust
use std::fs::File;
use std::io::ErrorKind;

let f = File::open("hello.txt");

let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error)
        }
    },
};
```

## 4. 辅助方法 (Helpers)

`match` 虽然强大，但有时过于冗长。`Result` 提供了许多辅助方法。

### 失败时 Panic: unwrap 和 expect

```rust
// 如果是 Err，直接 panic
let f = File::open("hello.txt").unwrap();

// 如果是 Err，panic 并附带自定义信息 (推荐)
let f = File::open("hello.txt").expect("Failed to open hello.txt");
```

### 失败时使用默认值: unwrap_or 和 unwrap_or_else

```rust
// 如果失败，返回提供的默认值
let v = result.unwrap_or(0);

// 如果失败，执行闭包计算默认值 (懒求值)
let v = result.unwrap_or_else(|err| {
    println!("Error: {}", err);
    0
});
```

## 5. 传播错误 (Propagating Errors)

与其在函数内部处理错误，不如将错误返回给调用者，让调用者决定如何处理。

### 原始方式

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // 提前返回错误
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

### 问号运算符 `?`

`?` 运算符大大简化了错误传播。如果 `Result` 是 `Ok`，它返回 `Ok` 中的值；如果是 `Err`，它直接从当前函数返回 `Err`。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // 链式调用
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

注意：`?` 只能用于返回类型为 `Result` (或 `Option`) 的函数中。

## 6. main 函数返回 Result

`main` 函数通常返回 `()`，但它也可以返回 `Result`。这允许我们在 `main` 中使用 `?`。

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

这里的 `Box<dyn Error>` 是一个 Trait 对象，意味着“任何类型的错误”。
