# Day 03: 函数与控制流

## 📝 学习目标
- 掌握 Rust 函数的定义与调用
- 理解参数传递与返回值机制
- 区分**语句** (Statements) 与**表达式** (Expressions)
- 熟练使用 `if` 条件表达式
- 掌握三种循环结构：`loop`, `while`, `for`

## 🎯 为什么要学这个
- **函数**是代码复用和模块化的基本单元。
- **表达式**是 Rust 的核心特性之一，理解它对于写出"Rust味"（Idiomatic）的代码至关重要。
- **控制流**决定了程序的逻辑走向，灵活运用循环和判断是解决问题的基础。

## 📖 核心概念

### 1. 函数 (Functions)
函数使用 `fn` 关键字定义。Rust 代码中的函数定义顺序不重要，只要编译器能找到即可。

#### 参数 (Parameters)
Rust 是强类型语言，必须在函数签名中显式指定每个参数的类型。

```rust
fn print_sum(a: i32, b: i32) {
    println!("Sum: {}", a + b);
}
```

#### 返回值 (Return Values)
函数可以有返回值。在箭头 `->` 后声明返回类型。
在 Rust 中，函数的最后一个表达式（没有分号）将作为返回值。也可以使用 `return` 关键字提前返回。

```rust
fn add_one(x: i32) -> i32 {
    x + 1 // 注意这里没有分号，这是一个表达式
}
```

### 2. 语句与表达式
这是 Rust 与很多其他语言的一个重要区别：
- **语句 (Statements)**：执行操作但不返回值。例如 `let x = 6;`。
- **表达式 (Expressions)**：进行计算并产生一个值。例如 `5 + 6`，或者 `{ let x = 3; x + 1 }`。

**关键规则**：如果表达式末尾加上分号，它就变成了语句，不再返回值（或者说返回单元类型 `()`）。

### 3. 控制流 (Control Flow)

#### `if` 表达式
`if` 在 Rust 中是一个表达式，这意味着它可以返回值。

```rust
let number = if condition { 5 } else { 6 };
```

#### 循环 (Loops)
Rust 提供了三种循环：

1.  **`loop`**: 无限循环。可以使用 `break` 退出，甚至通过 `break value` 返回值。
2.  **`while`**: 当条件为真时循环。
3.  **`for`**: 遍历集合或范围。这是 Rust 中最常用、最安全的循环方式。

## 💻 代码示例

### 示例 1: 具有返回值的函数
```rust
fn main() {
    let x = five();
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5 // 隐式返回
}
```

### 示例 2: `if` 作为表达式
```rust
fn main() {
    let condition = true;
    // if 分支和 else 分支返回的类型必须相同
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
```

### 示例 3: `loop` 循环返回值
```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // 返回 20
        }
    };
    println!("The result is {}", result);
}
```

### 示例 4: `for` 循环遍历
```rust
fn main() {
    // 遍历范围 (左闭右开)
    for number in 1..4 {
        println!("{}!", number);
    }

    // 遍历数组
    let a = [10, 20, 30];
    for element in a {
        println!("Value: {}", element);
    }
}
```

## 🏋️ 练习题

我们为你准备了专门的练习题来巩固这些概念。

- **练习 1**: 编写简单的数学函数
- **练习 2**: 使用 `if` 表达式处理条件
- **练习 3**: 斐波那契数列生成
- **练习 4**: 摄氏度与华氏度转换

👉 **[点击这里查看练习题](./exercises/README.md)**

## 🤔 常见问题 (FAQ)

### Q1: 为什么函数最后一行不加分号？
A: 在 Rust 中，不加分号表示这是一个**表达式**，它的值会被返回。如果加了分号，它就变成了**语句**，返回值变成 `()`（单元类型），这通常会导致类型不匹配的编译错误。

### Q2: `for` 循环中的 `1..4` 是什么？
A: 这是一个 `Range` 类型，表示从 1 到 3 的序列（包含 1，不包含 4）。如果想要包含 4，可以使用 `1..=4`。

### Q3: 什么时候用 `loop`，什么时候用 `while`？
A: 如果你需要一个无限循环（例如服务器的主循环），或者需要在循环结束时返回一个值，使用 `loop`。如果循环取决于某个条件，使用 `while`。但大多数情况下，遍历集合或范围时应优先使用 `for`，因为它更安全（不会越界）且性能更好（编译器优化）。

## 💡 最佳实践
- **优先使用表达式**：利用 Rust 基于表达式的特性，减少临时变量的声明。
- **优先使用 `for` 循环**：相比 `while` 和索引访问，`for` 循环更简洁且不易出错。
- **函数简短专注**：保持函数短小，只做一件事，有助于代码复用和测试。

## 🔗 扩展阅读
- [Rust 程序设计语言 - 函数](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust 程序设计语言 - 控制流](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

## 📚 本节要点回顾
- 函数参数必须标注类型。
- 函数最后一个表达式作为返回值（无分号）。
- `if` 是表达式，可以赋值给变量。
- `loop` 是无限循环，可以返回值。
- `for` 循环是遍历集合的首选。

## ⏭️ 下一步
有了变量、函数和控制流的基础，接下来我们将进入 Rust 最独特、最核心的概念——所有权。这是 Rust 内存安全的关键。

下一节: [Day 04: 所有权](../04.Ownership/README.md)
