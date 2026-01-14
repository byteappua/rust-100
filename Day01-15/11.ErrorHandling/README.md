# Day 11: 错误处理 (Error Handling)

Rust 将错误组合成两个主要类别：
1.  **可恢复错误 (Recoverable errors)**：通常代表向用户报告错误并重试的操作。例如：未找到文件。使用 `Result<T, E>`。
2.  **不可恢复错误 (Unrecoverable errors)**：是 bug 的症状。例如：尝试访问超过数组结尾的索引。使用 `panic!`。

## 1. panic! 与不可恢复错误

当 `panic!` 宏执行时，程序会打印一个错误信息，展开并清理栈数据，然后退出。

```rust
fn main() {
    panic!("crash and burn");
}
```

使用 `RUST_BACKTRACE=1` 环境变量运行程序可以获取回溯信息。

## 2. Result 与可恢复错误

`Result` 枚举定义如下：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 处理 Result

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
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

### unwrap 和 expect

帮助方法，如果 `Result` 是 `Err`，则调用 `panic!`。

```rust
let f = File::open("hello.txt").unwrap();
let f = File::open("hello.txt").expect("Failed to open hello.txt");
```

## 3. 传播错误

将错误返回给调用者。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

### 传播错误的简写：`?` 运算符

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

注意：`?` 运算符只能用于返回 `Result`、`Option` 或实现了 `FromResidual` 的类型的函数中。
