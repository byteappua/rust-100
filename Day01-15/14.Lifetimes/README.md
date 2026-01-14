# Day 14: 生命周期 (Lifetimes)

生命周期是 Rust 中最独特的概念之一。它是引用有效的作用域。

## 1. 悬垂引用 (Dangling References)

Rust 的借用检查器（Borrow Checker）确保所有引用都是有效的。

```rust
{
    let r;
    {
        let x = 5;
        r = &x;
    } // x 离开作用域被丢弃
    println!("r: {}", r); // 错误：r 引用了无效的内存
}
```

## 2. 泛型生命周期

当函数返回一个引用时，Rust 需要知道这个引用的生命周期与参数的生命周期有什么关系。

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这里的 `'a` 是生命周期注解。它告诉 Rust：
*   两个参数 `x` 和 `y` 存活的时间至少和 `'a` 一样长。
*   返回值的存活时间也至少和 `'a` 一样长。
*   实际上，返回值的生命周期是 `x` 和 `y` 中较短的那个。

## 3. 结构体定义中的生命周期

如果结构体持有引用，它必须拥有生命周期注解。

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

这意味着 `ImportantExcerpt` 的实例不能比其 `part` 字段引用的字符串存活得更久。

## 4. 生命周期省略规则 (Lifetime Elision Rules)

在某些常见情况下，编译器可以推断出生命周期，无需显式标注。

1.  每个是引用的参数都有其自己的生命周期参数。
2.  如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数。
3.  如果有多个输入生命周期参数，但其中一个是 `&self` 或 `&mut self`，那么 `self` 的生命周期被赋予所有输出生命周期参数。

## 5. 静态生命周期 ('static)

`'static` 生命周期存活于整个程序期间。所有的字符串字面值都拥有 `'static` 生命周期。

```rust
let s: &'static str = "I have a static lifetime.";
```
