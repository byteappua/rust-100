# Day 13: Trait (特征)

Trait (特征) 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。Trait 类似于其他语言中的接口（Interfaces），但有一些重要区别。

## 1. 定义 Trait

使用 `trait` 关键字定义一组方法签名。

```rust
pub trait Summary {
    fn summarize(&self) -> String;

    // 可以有默认实现
    fn summarize_author(&self) -> String {
        String::from("(Read more...)")
    }
}
```

## 2. 为类型实现 Trait

使用 `impl TraitName for TypeName` 语法。

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

### 孤儿规则 (Orphan Rule)

你只能在 trait 或类型至少有一个属于当前 crate 时，才能为该类型实现该 trait。
例如：你可以为自己的 `Tweet` 类型实现标准库的 `Display` trait。
但你不能为 `Vec<T>` (标准库) 实现 `Display` (标准库)，因为它们都不在你的 crate 中。

## 3. Trait 作为参数 (Trait Bounds)

### impl Trait 语法 (语法糖)

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

### Trait Bound 语法 (完整形式)

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

### 指定多个 Trait Bound

使用 `+` 连接。

```rust
pub fn notify(item: &(impl Summary + std::fmt::Display)) { ... }
```

### where 子句

当约束太长时，使用 `where` 使签名更清晰。

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ ... }
```

## 4. 返回实现了 Trait 的类型

使用 `impl Trait` 返回值。

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("..."),
        reply: false,
        retweet: false,
    }
}
```

**限制**：只能返回**一种**类型。如果你想根据条件返回 `NewsArticle` 或 `Tweet`，这行不通。

### Trait 对象 (Trait Objects)

如果需要返回多种类型，必须使用 Trait 对象（动态分发）。通常使用 `Box<dyn Trait>`。

```rust
fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle { ... })
    } else {
        Box::new(Tweet { ... })
    }
}
```

## 5. 派生 Trait (Derive)

对于一些常用的 Trait，Rust 提供了宏来自动实现它们。

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

## 6. 父 Trait (Supertraits)

有时候你需要一个 Trait 依赖于另一个 Trait。

```rust
use std::fmt::Display;

trait OutlinePrint: Display { // 要求实现 OutlinePrint 的类型必须也实现 Display
    fn outline_print(&self) {
        let output = self.to_string(); // 因为有 Display，所以可以调用 to_string
        let len = output.len();
        println!("{}n{}n{}", "*".repeat(len + 4), format!("* {} *", output), "*".repeat(len + 4));
    }
}
```
