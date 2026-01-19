# Day 13: Trait (特征)

## 📝 学习目标
- 理解 Trait 的概念（类似接口）
- 掌握如何定义 Trait 和为类型实现 Trait
- 理解 Trait Bounds (约束) 和 `impl Trait` 语法
- 掌握孤儿规则 (Orphan Rule)
- 了解常用的派生 Trait (`#[derive]`)

## 🎯 为什么要学这个
- **多态**: Trait 是 Rust 实现多态的核心机制。它允许你编写能够处理任何具有特定行为（方法）的类型的代码。
- **抽象**: 定义共享的行为接口，而不关心具体实现细节。
- **约束**: 限制泛型参数必须具备的功能（如“必须能被打印”）。

## 📖 核心概念

### 1. 定义与实现
Trait 定义了一组方法签名。

```rust
pub trait Summary {
    fn summarize(&self) -> String;

    // 默认实现
    fn summarize_author(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

### 2. Trait 作为参数
我们可以编写接受任何实现了特定 Trait 的类型的函数。

```rust
// 语法糖
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//完整形式 (Trait Bound)
pub fn notify<T: Summary>(item: &T) { ... }
```

### 3. 多重约束与 Where 子句
如果需要多个 Trait，用 `+` 连接。如果太长，用 `where`。

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ ... }
```

### 4. 孤儿规则 (Orphan Rule)
为了保证一致性，你不能为 **外部类型** 实现 **外部 Trait**。
- 可以在这里为 `Vec<T>` (外部) 实现 `Summary` (本地)。
- 可以在这里为 `Tweet` (本地) 实现 `Display` (外部)。
- **不能** 在这里为 `Vec<T>` (外部) 实现 `Display` (外部)。

### 5. 返回 impl Trait
函数可以返回实现了某个 Trait 的类型，但实际返回的具体类型必须是确定的（只能是一种）。

```rust
fn returns_summarizable() -> impl Summary {
    Tweet { ... }
}
```

## 💻 代码示例

### 示例 1: 基本实现
```rust
trait Speak {
    fn say_hello(&self);
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn say_hello(&self) { println!("Woof!"); }
}

impl Speak for Cat {
    fn say_hello(&self) { println!("Meow!"); }
}

fn make_it_speak(pet: &impl Speak) {
    pet.say_hello();
}

fn main() {
    let d = Dog;
    let c = Cat;
    make_it_speak(&d);
    make_it_speak(&c);
}
```

### 示例 2: 使用 Derive 宏
Rust 编译器可以自动为我们实现一些标准 Trait。

```rust
#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone(); // Clone
    println!("{:?}", p1); // Debug
    println!("Equal? {}", p1 == p2); // PartialEq
}
```

## 🏋️ 练习题

我们准备了练习题来帮助你掌握 Trait 的使用。

- **练习 1**: 定义和实现 Trait
- **练习 2**: 使用 Trait 作为函数参数
- **练习 3**: 默认实现与重写
- **练习 4**: 实现标准库 Trait (`Display`)

👉 **[点击这里查看练习题](./exercises/README.md)**

## 🤔 常见问题 (FAQ)

### Q1: Trait 和 Java/C# 的 Interface 有什么区别？
A: 非常相似，但 Trait 可以包含默认实现（Java 8+ 接口也可以）。最大的区别在于 Trait 可以作为泛型约束 (Bounds)，不仅用于动态分发，更多用于静态单态化。此外，Trait 不支持字段（数据），只定义行为。

### Q2: 什么是关联类型 (Associated Types)？
A: 这是进阶话题 (Day 28)。简单来说，它是 Trait 定义中的类型占位符，例如 `Iterator` trait 有一个 `type Item;`，实现时指定具体的 Item 类型。

## 💡 最佳实践
- **优先使用标准 Trait**: 如 `Display`, `Debug`, `Default`, `Clone`, `Copy` 等。遵循 Rust 的惯例会让你的类型更好用。
- **使用 Derive**: 只要可能，就通过 `#[derive(...)]` 自动实现标准 Trait，减少样板代码。
- **单一职责**: 定义小而专注的 Trait，而不是大而全的 Trait。

## 🔗 扩展阅读
- [Rust 程序设计语言 - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

## ⏭️ 下一步
现在我们理解了泛型和 Trait。但还有一个棘手的问题：引用在什么时候是有效的？如果引用的数据被释放了怎么办？这就是生命周期要解决的问题。

下一节: [Day 14: 生命周期](../14.Lifetimes/README.md)
