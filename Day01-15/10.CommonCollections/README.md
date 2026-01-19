# Day 10: 常用集合 (Common Collections)

## 📝 学习目标
- 掌握 **Vector (`Vec<T>`)** 的增删改查操作
- 理解 **String** 的编码机制与操作限制
- 熟练使用 **HashMap (`HashMap<K, V>`)** 进行键值存储
- 理解这些集合在内存中的布局（堆内存）

## 🎯 为什么要学这个
Rust 的标准库提供了许多强大的数据结构，称为"集合"。与内置的数组和元组不同：
- 它们的数据存储在 **堆 (Heap)** 上。
- 它们的大小是 **动态** 的（运行时可变）。
- 它们是处理不定长数据的基石。

## 📖 核心概念

### 1. Vector (向量)
`Vec<T>` 是一个可增长的数组。它在内存中是连续存储的。

- **创建**: `Vec::new()` 或 `vec![1, 2, 3]`。
- **访问**: 下标索引 `&v[i]` (越界 Panic) 或 `v.get(i)` (返回 `Option`)。
- **遍历**: `for x in &v`。

```rust
let mut v = vec![1, 2, 3];
v.push(4);      // 添加
v.pop();        // 删除最后一个并返回 Option<T>
```

### 2. String (字符串)
Rust 的 `String` 是 UTF-8 编码的字节序列。

- **本质**: `String` 是 `Vec<u8>` 的封装。
- **不支持索引**: `s[0]` 是非法的，因为一个 UTF-8 字符可能占用 1-4 个字节，简单的字节索引无法准确对应字符。
- **操作**: `push_str`, `+` (拼接), `format!`。
- **遍历**:
    - `s.chars()`: 遍历 Unicode 标量值 (char)。
    - `s.bytes()`: 遍历原始字节。

```rust
let s = String::from("你好");
// s.len() 是 6 (每个汉字 3 字节)
// s.chars().count() 是 2
```

### 3. HashMap (哈希表)
`HashMap<K, V>` 存储键值对映射。

- **所有权**:
    - 对于实现了 `Copy` 的类型 (如 `i32`)，值会被拷贝。
    - 对于 `String` 等类型，值的所有权会被移动进 HashMap。
- **更新**: `insert` (覆盖), `entry(...).or_insert(...)` (不存在则插入)。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
```

## 💻 代码示例

### 示例 1: Vector 操作与枚举
Vector 只能存储同一种类型。如果需要存储不同类型，可以使用枚举。

```rust
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        println!("{:?}", cell);
    }
}
```

### 示例 2: HashMap 统计词频
```rust
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
```

## 🏋️ 练习题

我们准备了练习题来帮助你掌握这三种集合的使用。

- **练习 1**: 使用 Vector 进行统计（平均数、中位数、众数）
- **练习 2**: Pig Latin (猪拉丁语) 转换工具
- **练习 3**: 员工部门管理系统 (HashMap 综合应用)

👉 **[点击这里查看练习题](./exercises/README.md)**

## 🤔 常见问题 (FAQ)

### Q1: `&String` 和 `&str` 有什么区别？
A: `String` 是堆上分配的、可变的字符串对象。`&str` 是字符串切片（通常是对 String 或字面量的引用），是不可变的视图。函数参数通常建议写 `&str` 以兼容两种类型（`&String` 会自动强转为 `&str`）。

### Q2: 为什么 `s1 + &s2` 中 `s1` 必须被移动？
A: `+` 运算符调用的是 `add` 方法：`fn add(self, s: &str) -> String`。第一个参数是 `self`，意味着它获取了 `s1` 的所有权。为了效率，它直接在 `s1` 的缓冲区上追加 `s2` 的内容，并返回修改后的 `s1`，避免了重新分配新内存。

### Q3: 什么时候用数组 `[T; N]`，什么时候用 `Vec<T>`？
A: 如果数据数量固定且较小，用数组（栈分配，更快）。如果数量不确定或很大，用 Vector。

## 💡 最佳实践
- **预分配容量**: 如果知道大概要存多少元素，使用 `Vec::with_capacity(n)` 或 `HashMap::with_capacity(n)` 可以减少内存重新分配的次数，提高性能。
- **Entry API**: 修改 HashMap 时优先使用 `entry` API，代码更简洁且效率更高（只计算一次哈希值）。

## 🔗 扩展阅读
- [Rust 程序设计语言 - 常见集合](https://doc.rust-lang.org/book/ch08-00-common-collections.html)

## ⏭️ 下一步
现在我们已经掌握了基础语法和常用数据结构。但在编程中，错误是不可避免的。Rust 有一套独特且安全的错误处理机制。

下一节: [Day 11: 错误处理](../11.ErrorHandling/README.md)
