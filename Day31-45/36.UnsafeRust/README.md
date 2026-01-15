# Day 36: Unsafe Rust 介绍

Rust 的核心承诺是内存安全。然而，Rust 也提供了一种逃生舱（Escape Hatch），称为 **Unsafe Rust**。它允许你绕过编译器的某些安全检查，以获得对内存和底层硬件的更多控制。

## 为什么需要 Unsafe Rust？

1.  **底层系统编程**：直接与操作系统或硬件交互。
2.  **性能优化**：在借用检查器无法理解代码安全性的情况下，手动管理内存以提升性能。
3.  **互操作性 (FFI)**：调用 C 语言或其他语言的代码。

## Unsafe 的 5 种超能力

在 `unsafe` 块中，你可以执行以下 5 种在安全 Rust 中被禁止的操作：

1.  解引用裸指针。
2.  调用不安全的函数或方法。
3.  访问或修改可变静态变量。
4.  实现不安全的 Trait。
5.  访问 `union` 的字段。

> 注意：`unsafe` 并不会关闭借用检查器或禁用其他 Rust 安全检查。它只是赋予了你上述特定的权限。你仍然需要对自己写的代码的安全性负责。

## 1. 裸指针 (Raw Pointers)

裸指针 (`*const T` 和 `*mut T`) 类似于 C 语言的指针。与引用不同：
- 允许忽略借用规则（可以同时拥有多个可变指针）。
- 不保证指向有效的内存。
- 允许为空。
- 不实现自动清理。

```rust
let mut num = 5;

// 创建裸指针是安全的
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    // 解引用裸指针是不安全的
    println!("r1 is: {}", *r1);
    *r2 = 10;
}
```

## 2. 调用不安全的函数

如果一个函数被标记为 `unsafe`，说明编译器无法验证其安全性，需要程序员担保。

```rust
unsafe fn dangerous() {}

fn main() {
    unsafe {
        dangerous();
    }
}
```

## 3. 创建安全抽象

通常我们将 unsafe 代码封装在安全的 API 后面。例如，标准库的 `split_at_mut` 方法内部使用了 unsafe 代码来分割切片，但对外提供了安全的接口。

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

## 4. 访问可变静态变量

全局变量在 Rust 中称为静态变量 (`static`)。如果它们是可变的 (`static mut`)，多线程访问可能会导致数据竞争，因此访问它们必须在 `unsafe` 块中。

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
```

## 5. FFI (外部函数接口)

调用 C 语言库函数需要使用 `extern` 块，并且调用过程是 unsafe 的。

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3: {}", abs(-3));
    }
}
```
