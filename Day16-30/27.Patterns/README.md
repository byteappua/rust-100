# Day 27: 模式与匹配 (Patterns and Matching)

Rust 的模式匹配功能非常强大，不仅限于 `match` 表达式。

## 1. 模式的使用场景

*   **match 分支**: 每个分支都是一个模式。
*   **if let**: 只处理一种匹配情况。
*   **while let**: 只要模式匹配就一直循环。
*   **for 循环**: `for (index, value) in v.iter().enumerate()` 中的 `(index, value)` 也是模式。
*   **let 语句**: `let (x, y) = (1, 2);` 也是解构模式。
*   **函数参数**: `fn print_coordinates(&(x, y): &(i32, i32))`。

## 2. Refutability (可反驳性)

*   **Irrefutable (不可反驳)**: 任何可能的值都能匹配。例如 `let x = 5;` 中的 `x`。
*   **Refutable (可反驳)**: 某些值可能不匹配。例如 `if let Some(x) = a_value` 中的 `Some(x)`（如果值是 `None` 就不匹配）。

`let` 语句、函数参数、`for` 循环必须接受不可反驳的模式。

## 3. 模式语法

### 解构

可以解构结构体、枚举、元组。

```rust
struct Point { x: i32, y: i32 }
let p = Point { x: 0, y: 7 };

match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}
```

### 忽略值

*   `_`: 忽略单个值。
*   `..`: 忽略剩余值。

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, .., last) => {
        println!("Some numbers: {}, {}", first, last);
    }
}
```

### 匹配守卫 (Match Guards)

`match` 分支模式后可以跟一个 `if` 条件。

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

### @ 绑定

允许我们在创建一个存放值的变量的同时测试其值是否匹配模式。

```rust
match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    // ...
}
```
