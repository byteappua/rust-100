# Day 38: FFI - 与 C 语言交互

Rust 拥有强大的**外部函数接口 (Foreign Function Interface, FFI)**，这使得它可以无缝地调用 C 语言代码，或者被 C 语言调用。这对于利用现有的 C 库生态系统或将 Rust 集成到现有的 C/C++ 项目中至关重要。

## 核心概念

1.  **`extern "C"` 块**: 用于在 Rust 中声明 C 语言定义的函数签名。
2.  **`unsafe` 块**: 调用外部函数被认为是不安全的，因为编译器无法检查 C 代码的内存安全性和类型正确性。
3.  **`#[repr(C)]`**: 告诉 Rust 编译器按照 C 语言的规则来布局结构体内存，以确保两边看到的数据结构一致。
4.  **`libc` crate**: 提供了 C 标准库中的类型定义（如 `c_int`, `c_void` 等），确保跨平台兼容性。

## 本日实战：Rust 调用 C 代码

我们将创建一个包含 C 代码的 Cargo 项目，并使用 `cc` crate 在构建时编译 C 代码，然后在 Rust 中调用它。

### 1. 编译 C 代码 (`build.rs`)

Cargo 允许通过 `build.rs` 脚本自定义构建过程。我们使用 `cc` crate 来编译 `src/mylib.c`。

```toml
# Cargo.toml
[build-dependencies]
cc = "1.0"
```

```rust
// build.rs
fn main() {
    cc::Build::new()
        .file("src/mylib.c")
        .compile("mylib");
}
```

### 2. 声明与调用 (`main.rs`)

在 Rust 中，我们需要使用 `extern "C"` 声明 C 函数。

```rust
use std::os::raw::c_int;

extern "C" {
    fn c_add(a: c_int, b: c_int) -> c_int;
}

fn main() {
    unsafe {
        println!("Result: {}", c_add(10, 20));
    }
}
```

### 3. 处理字符串 (`CString` vs `CStr`)

Rust 的字符串 (`String`, `&str`) 不是以空字符结尾的，而 C 字符串是 (`char*`)。
*   **`std::ffi::CString`**: 用于创建拥有所有权的、兼容 C 的字符串（自动添加 `\0`）。用于传给 C。
*   **`std::ffi::CStr`**: 用于借用 C 字符串。用于从 C 接收。

```rust
use std::ffi::CString;

let c_name = CString::new("Rust").expect("Failed");
unsafe {
    c_print_hello(c_name.as_ptr());
}
```

### 4. 结构体布局 (`#[repr(C)]`)

如果要在 Rust 和 C 之间传递结构体，必须保证内存布局一致。

```rust
#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}
```

> **注意**: 在实际项目中，手动维护 FFI 绑定非常繁琐且容易出错。通常建议使用 **`bindgen`** 工具自动生成 Rust 绑定代码。
