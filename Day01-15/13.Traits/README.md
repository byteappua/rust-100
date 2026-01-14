# Day 13: Trait (特征)

Trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。Trait 类似于其他语言中的接口（Interfaces），但有一些区别。

## 1. 定义 Trait

使用 `trait` 关键字定义。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

## 2. 为类型实现 Trait

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

## 3. 默认实现

可以在 Trait 定义中提供默认行为。

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

## 4. Trait 作为参数 (Trait Bounds)

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

这实际上是 Trait Bound 语法的语法糖：

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

### 指定多个 Trait Bound

```rust
pub fn notify(item: &(impl Summary + Display)) {
```

### where 子句

当 Trait Bound 太多时，使用 `where` 从句可以使代码更清晰。

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

## 5. 返回实现了 Trait 的类型

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

注意：这种方式只适用于返回单一类型的情况。
