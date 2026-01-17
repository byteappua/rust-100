# Day 08: 枚举与模式匹配

## 📝 学习目标
- 理解枚举 (`Enum`) 的定义和使用场景
- 掌握 `Option<T>` 枚举及其重要性
- 掌握 `match` 控制流运算符
- 理解模式匹配中的绑定值和穷尽性
- 掌握 `if let` 语法糖

## 🎯 为什么要学这个
枚举允许你列举所有可能的值来定义一个类型。
- **表达能力强**：枚举可以清晰地表达"一组相关项中的某一项"（如 IP 地址是 V4 或 V6）。
- **类型安全**：Rust 的 `Option<T>` 替代了其他语言中的 `null`，迫使你在编译期处理"无值"的情况，彻底杜绝了"空指针异常"。
- **强大的控制流**：`match` 模式匹配比 `switch` 强大得多，它能解构数据、检查完整性。

## 📖 核心概念

### 1. 定义枚举
枚举 (Enumerations) 包含多个变体 (Variants)。

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

**枚举变体可以包含数据**：
每个变体可以包含不同类型和数量的数据。
```rust
enum Message {
    Quit,                       // 没有数据
    Move { x: i32, y: i32 },    // 包含匿名结构体
    Write(String),              // 包含 String
    ChangeColor(i32, i32, i32), // 包含三个 i32
}
```

### 2. Option 枚举
Rust 没有 `null`。它使用 `Option<T>` 标准库枚举来表示一个值可能存在，也可能不存在。

```rust
enum Option<T> {
    Some(T),
    None,
}
```
`Some(T)` 表示有值，`None` 表示无值。
使用 `Option` 的好处是，你不能直接把 `Option<T>` 当作 `T` 来使用，必须显式地处理 `None` 的情况，这保证了安全。

### 3. match 控制流运算符
`match` 允许我们将一个值与一系列的模式进行比较，并根据匹配的模式执行代码。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

**绑定值**：
匹配分支可以绑定到匹配模式的部分值。
```rust
match message {
    Message::Write(text) => println!("Text: {}", text),
    // ...
}
```

**匹配是穷尽的 (Exhaustive)**：
必须处理所有可能的情况，否则编译不通过。
可以使用 `_` 通配符来处理"其他所有情况"。
```rust
let u8_value = 0u8;
match u8_value {
    1 => println!("one"),
    3 => println!("three"),
    _ => (),
}
```

### 4. if let 简单控制流
当你只关心一种匹配情况而忽略其他情况时，`if let` 更加简洁。

```rust
let some_u8_value = Some(3u8);

// 使用 match
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}

// 使用 if let
if let Some(3) = some_u8_value {
    println!("three");
}
```
`if let` 放弃了穷尽性检查，换取了简洁。

## 💻 代码示例

### 示例: 处理 Option
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

## 🏋️ 练习题

我们为你准备了关于枚举定义、Option 使用和 match 匹配的练习。

- **练习 1**: 定义消息枚举
- **练习 2**: Option 加法
- **练习 3**: 硬币分类器

👉 **[点击这里查看练习题](./exercises/README.md)**

## 🤔 常见问题 (FAQ)

### Q1: 什么时候用 struct，什么时候用 enum？
A: 如果你的数据是"并且"的关系（既有名字又有年龄），用 struct。如果你的数据是"或者"的关系（要是 IPv4 要么是 IPv6），用 enum。

### Q2: 为什么 `Option::Some(5)` 可以直接写成 `Some(5)`？
A: 因为 `Option` 及其变体 `Some`, `None` 被包含在了 Rust 的预导入模块 (prelude) 中，不需要手动 `use`。

### Q3: `if let` 和 `match` 性能有区别吗？
A: 几乎没有区别。编译器通常会优化它们生成相似的汇编代码。选择依据主要是代码的可读性和是否需要穷尽性检查。

## 💡 最佳实践
- **利用类型系统**：尽量使用枚举来表示状态机的状态，利用编译器检查确保状态转换正确。
- **优先 match**：对于复杂的条件判断，`match` 比一堆 `if else` 更清晰且更安全（穷尽性检查）。
- **处理 None**：不要急着 `unwrap()`，尽量使用 `match` 或 `if let` 优雅地处理 `None`。

## 🔗 扩展阅读
- [Rust 程序设计语言 - 枚举与模式匹配](https://doc.rust-lang.org/book/ch06-00-enums.html)

## 📚 本节要点回顾
- `enum` 定义枚举。
- `Option<T>` 处理空值。
- `match` 进行模式匹配，必须穷尽。
- `_` 通配符匹配其余情况。
- `if let` 处理只关心一种情况的场景。

## ⏭️ 下一步
现在我们已经掌握了变量、结构体和枚举。随着代码量的增加，我们需要更好的组织代码。

下一节: [Day 09: 模块系统](../09.Modules/README.md)
