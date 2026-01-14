# Day 02: 变量与数据类型

## 1. 变量与可变性

在 Rust 中，变量默认是**不可变**（Immutable）的。这是 Rust 推动编写安全且易于并发代码的重要方式之一。

### 声明变量

使用 `let` 关键字声明变量：

```rust
let x = 5;
println!("The value of x is: {}", x);
// x = 6; // 这行代码会报错，因为 x 是不可变的
```

### 可变变量

如果你想让变量可变，需要在 `let` 后加上 `mut` 关键字：

```rust
let mut x = 5;
println!("The value of x is: {}", x);
x = 6;
println!("The value of x is: {}", x);
```

### 常量

常量使用 `const` 关键字声明，必须注明类型，且只能被设置为常量表达式，不能是函数调用的结果。

```rust
const MAX_POINTS: u32 = 100_000;
```

### 隐藏 (Shadowing)

你可以声明和前面变量同名的新变量，这被称为“隐藏”。通常用于类型转换。

```rust
let x = 5;
let x = x + 1; // 新的 x 隐藏了旧的 x
let x = "spaces"; // 甚至可以改变类型
```

## 2. 数据类型

Rust 是静态类型语言。如果不明确，编译器通常可以推断出类型。

### 标量类型 (Scalar Types)

标量类型代表一个单独的值。

1.  **整型 (Integer)**:
    *   有符号: `i8`, `i16`, `i32` (默认), `i64`, `i128`, `isize`
    *   无符号: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

2.  **浮点型 (Floating-Point)**:
    *   `f32`
    *   `f64` (默认)

3.  **布尔型 (Boolean)**:
    *   `bool` (值为 `true` 或 `false`)

4.  **字符类型 (Character)**:
    *   `char` (4字节，代表 Unicode 标量值)

### 复合类型 (Compound Types)

1.  **元组 (Tuple)**:
    *   可以将多个不同类型的值组合在一起。
    *   长度固定。

    ```rust
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // 解构
    let five_hundred = tup.0; // 索引访问
    ```

2.  **数组 (Array)**:
    *   包含多个相同类型的值。
    *   长度固定。

    ```rust
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    ```

## 3. 练习

创建一个名为 `variables` 的项目，尝试以下操作：
1.  定义一个不可变变量，尝试修改它，观察编译器错误。
2.  定义一个可变变量，修改它的值。
3.  定义一个元组，包含你的年龄（整数）和身高（浮点数），并打印出来。
