# Day 17: 迭代器 (Iterators)

## 📝 学习目标
- 理解 `Iterator` trait 及其核心方法 `next`。
- 掌握消费适配器（Consuming Adaptors）如 `sum`, `collect`。
- 掌握迭代器适配器（Iterator Adaptors）如 `map`, `filter`, `zip`。
- 能够实现自定义迭代器。

## 🎯 为什么要学这个
迭代器模式是 Rust 处理序列数据的主要方式。它不仅语法简洁，而且在 Rust 中是**零成本抽象**（Zero-Cost Abstractions）。通常情况下，使用迭代器的速度与手写底层 `for` 循环一样快，甚至更快（因为编译器能更好地优化边界检查）。

## 📖 核心概念

### 1. Iterator Trait
所有迭代器都实现了 `Iterator` trait。

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ... 其它方法有默认实现
}
```

*   `next` 方法返回 `Option<Self::Item>`。如果返回 `Some(item)`，则处理下一个元素；如果返回 `None`，则迭代结束。

### 2. 创建迭代器
*   `iter()`: 创建不可变引用的迭代器。
*   `iter_mut()`: 创建可变引用的迭代器。
*   `into_iter()`: 创建获取所有权的迭代器。

### 3. 消费适配器 (Consuming Adaptors)
这些方法会调用 `next`，消耗掉迭代器，并生成一个最终结果。
*   `collect()`: 将迭代器转换成集合（如 `Vec`, `HashMap`）。
*   `sum()`, `product()`: 求和、求积。
*   `count()`: 计算元素个数。
*   `for_each()`: 对每个元素执行操作。

### 4. 迭代器适配器 (Iterator Adaptors)
这些方法将当前迭代器转换成另一个迭代器。它们是**惰性**的（Lazy），必须配合消费适配器才会执行。
*   `map()`: 转换元素。
*   `filter()`: 筛选元素。
*   `zip()`: 将两个迭代器合并为一个元组迭代器。
*   `chain()`: 连接两个迭代器。
*   `enumerate()`: 添加索引。

## 💻 代码示例

### 示例 1: 基本使用

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // 惰性，不产生任何操作

    for val in v1_iter { // 这里隐式调用 next
        println!("Got: {}", val);
    }
}
```

### 示例 2: map 和 filter

```rust
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // map 和 filter 都是惰性的，必须最后调用 collect
    let v2: Vec<_> = v1.iter()
                       .map(|x| x + 1)
                       .filter(|x| x % 2 == 0)
                       .collect();

    println!("{:?}", v2); // [2, 4]
}
```

## 🏋️ 练习题

### 练习 1: 平方和
编写一个函数，接受一个 `&[i32]`，返回其中所有偶数的平方和。使用迭代器链式调用实现。
[查看参考答案](./exercises/solutions/exercise1_solution.rs)

### 练习 2: 自定义迭代器
实现一个 `Counter` 结构体，从 1 数到 5。
[查看参考答案](./exercises/solutions/exercise2_solution.rs)

### 练习 3: 复杂链式操作
给定一个字符串向量，过滤出长度大于 3 的字符串，将其转换为大写，并用逗号连接成一个单一的字符串。
[查看参考答案](./exercises/solutions/exercise3_solution.rs)

## 🤔 常见问题

### Q1: 什么时候用 `iter()` vs `into_iter()`?
A:
- `iter()`: 当你想遍历数据的引用（借用），保留原始数据时。
- `into_iter()`: 当你想获取数据的所有权（移动），原始数据不再需要时。

### Q2: 为什么 `collect()` 需要类型标注？
A: `collect()` 可以将迭代器转换成多种不同的集合（Vec, LinkedList, HashMap 等）。编译器通常无法推断你想转换成哪种具体类型，所以需要显式标注 `Vec<_>` 或 `Vec<i32>`。

## 🔗 扩展阅读
- [Rust Book: Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Standard Library: Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
