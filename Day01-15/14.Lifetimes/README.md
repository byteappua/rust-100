# Day 14: 生命周期 (Lifetimes)

生命周期是 Rust 中最独特、也最让初学者困惑的概念之一。简单来说，生命周期就是**引用保持有效的作用域**。Rust 的借用检查器使用生命周期来确保所有的借用都是有效的。

## 1. 为什么需要生命周期？

主要为了避免**悬垂引用**（Dangling References）。

```rust
{
    let r;                // ---------+-- r 的生命周期 'a
    {                     //          |
        let x = 5;        // -+-- x 的生命周期 'b
        r = &x;           //  |
    }                     // -+       x 结束，内存被释放
    println!("r: {}", r); //          r 引用了无效内存！编译错误
}                         // ---------+
```

Rust 编译器会拒绝这段代码，因为它发现 `r` 的生命周期 `'a` 比 `x` 的生命周期 `'b` 长。被引用的对象 `x` 必须活得比引用它的 `r` 更久。

## 2. 函数中的泛型生命周期

当函数接收并返回引用时，编译器通常需要帮助来理解引用之间的关系。

```rust
// 错误代码
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
```

编译器不知道返回的引用是指向 `x` 还是 `y`，因此也就不知道返回值的生命周期应该是多长。

### 生命周期注解语法

*   以 `'` 开头，通常使用 `'a`，`'b` 等非常短的名字。
*   注解放在 `&` 之后，类型之前。

```rust
&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
```

### 修正 longest 函数

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这告诉编译器：
1.  函数有两个参数 `x` 和 `y`，它们的生命周期至少都为 `'a`。
2.  返回值的生命周期也至少为 `'a`。
3.  实际上，`'a` 等于 `x` 和 `y` 中生命周期**较短**的那个。

**重要理解**：生命周期注解**不会改变**任何值的实际存活时间。它只是为了让借用检查器能验证这些引用是安全的。

## 3. 结构体定义中的生命周期

如果结构体持有引用，它必须在定义中包含生命周期注解。

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    // i 持有 novel 一部分的引用
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

这就意味着：`ImportantExcerpt` 的实例 `i` 不能比它引用的 `part` (即 `novel` 的一部分) 活得更久。

## 4. 方法定义中的生命周期

为带有生命周期的结构体实现方法时，语法如下：

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

这里应用了生命周期省略规则（见下文），所以 `announce_and_return_part` 的返回值生命周期被推断为与 `&self` 相同。

## 5. 生命周期省略规则 (Lifetime Elision Rules)

在 Rust 的早期版本中，每个引用都需要显式标注生命周期。后来，Rust 团队发现许多模式是重复的，于是制定了规则，让编译器在这些特定情况下自动推断生命周期。

**三条规则**：
1.  每一个引用参数都有它自己的生命周期参数。（例如 `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`）
2.  如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数。（例如 `fn foo<'a>(x: &'a i32) -> &'a i32`）
3.  如果有多个输入生命周期参数，但其中一个是 `&self` 或 `&mut self`，那么 `self` 的生命周期被赋予所有输出生命周期参数。

## 6. 静态生命周期 ('static)

`'static` 是一个特殊的生命周期，表示引用可以在**整个程序期间**存活。

*   所有的字符串字面值都拥有 `'static` 生命周期：
    ```rust
    let s: &'static str = "I have a static lifetime.";
    ```
*   `'static` 也可以用于 trait bound，表示类型不包含任何非静态的引用。

## 7. 结合泛型类型参数、Trait Bounds 和生命周期

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
