# Day 31: 文件输入与输出 (File I/O)

Rust 的标准库 `std::fs` 和 `std::io` 提供了处理文件系统的功能。

## 1. 打开与创建文件

*   `File::open(path)`: 以只读模式打开文件。
*   `File::create(path)`: 创建文件（如果存在则截断/覆盖）。
*   `OpenOptions`: 提供更多控制（如追加模式）。

```rust
use std::fs::File;
use std::fs::OpenOptions;

let f = File::open("hello.txt")?;
let f = File::create("hello.txt")?;
let f = OpenOptions::new().append(true).open("hello.txt")?;
```

## 2. 读取文件

*   `fs::read_to_string(path)`: 一次性读取整个文件为字符串。
*   `fs::read(path)`: 一次性读取整个文件为字节向量 `Vec<u8>`。
*   `BufReader`: 缓冲读取，适合按行读取。

```rust
use std::io::{BufRead, BufReader};

let f = File::open("hello.txt")?;
let reader = BufReader::new(f);

for line in reader.lines() {
    println!("{}", line?);
}
```

## 3. 写入文件

*   `write_all`: 写入字节切片。
*   `writeln!`: 宏，方便写入字符串并换行。

```rust
use std::io::Write;

let mut f = File::create("output.txt")?;
f.write_all(b"Hello")?;
writeln!(f, "World")?;
```
