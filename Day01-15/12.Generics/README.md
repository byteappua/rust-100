# Day 12: 泛型 (Generics)

## 📝 学习目标
- 理解泛型的概念与作用
- 掌握在函数、结构体、枚举和方法中定义泛型
- 理解 Rust 的 **单态化 (Monomorphization)** 机制（零成本抽象）
- 能够阅读和编写带有基本 Trait Bound 的泛型代码

## 🎯 为什么要学这个
- **减少重复代码**: 不需要为 i32, f64, String 分别写一个排序函数，只需写一个泛型排序函数。
- **类型安全**: 编译器确保泛型参数的使用是合法的。
- **高性能**: Rust 的泛型在编译时展开，运行时没有任何性能损耗。

## 📖 核心概念

### 1. 什么是泛型？
泛型是具体类型的占位符。就像函数参数是值的占位符一样，泛型参数是类型的占位符。通常使用 `T` (Type) 来表示。

### 2. 在哪里使用泛型？
- **函数**: `fn largest<T>(list: &[T]) -> T { ... }`
- **结构体**: `struct Point<T> { x: T, y: T }`
- **枚举**: `enum Result<T, E> { Ok(T), Err(E) }`
- **方法**: `impl<T> Point<T> { ... }`

### 3. 单态化 (Monomorphization)
这是 Rust 泛型高性能的秘密。
当编译泛型代码时，Rust 会寻找所有使用该泛型的地方，并为每个具体类型生成专门的代码副本。
例如，如果你用了 `Option<i32>` 和 `Option<f64>`，编译器会生成两个不同的枚举定义。这会稍微增加二进制文件大小，但运行时速度极快（静态分发）。

## 💻 代码示例

### 示例 1: 泛型结构体
```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let error = Point { x: 5, y: 4.0 }; // 错误！T 必须是相同类型
}
```

### 示例 2: 多个泛型参数
```rust
struct PointBoth<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both = PointBoth { x: 5, y: 4.0 }; // 合法
}
```

### 示例 3: 泛型方法
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 也可以为特定类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

## 🏋️ 练习题

我们准备了练习题来帮助你掌握泛型的使用。

- **练习 1**: 简单的泛型封装
- **练习 2**: 泛型函数实现
- **练习 3**: 混合泛型结构体操作

👉 **[点击这里查看练习题](./exercises/README.md)**

## 🤔 常见问题 (FAQ)

### Q1: `impl Point<T>` 和 `impl<T> Point<T>` 有什么区别？
A: `impl<T> Point<T>` 读作“为任意类型 T 实现 Point<T>”。第一个 `<T>` 声明了 T 是一个泛型参数，而不是一个具体的类型名称。如果没有第一个 `<T>`，编译器会以为 `T` 是某个具体的 struct 或 enum 的名字。

### Q2: 泛型会拖慢编译速度吗？
A: 会。因为编译器需要为每个具体类型生成代码副本（单态化），这增加了编译工作量。但它换来的是运行时的极致性能。

## 💡 最佳实践
- **命名规范**: 单个泛型参数通常叫 `T`。如果有多个，通常用 `T, U, V` 或更具描述性的名字（如 `K, V` 用于 Key-Value）。
- **约束**: 虽然这节课还没深入讲 Trait Bound，但要注意，如果你在泛型函数中使用了 `>` 运算符，你需要告诉编译器 `T` 是可以比较的（`T: PartialOrd`）。

## 🔗 扩展阅读
- [Rust 程序设计语言 - 泛型](https://doc.rust-lang.org/book/ch10-01-syntax.html)

## ⏭️ 下一步
泛型让我们定义了“某种类型”，但如果我们想限制这种类型必须具有“某些行为”（比如能被打印、能被比较）怎么办？这就是 Trait 的作用。

下一节: [Day 13: Trait](../13.Traits/README.md)
