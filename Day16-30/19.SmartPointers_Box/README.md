# Day 19: 智能指针 - Box<T>

智能指针（Smart Pointers）是行为类似于指针的数据结构，但它们通常拥有元数据和附加功能。最显著的区别是智能指针通常**拥有**它们指向的数据。

常见的智能指针：
*   `Box<T>`: 用于在堆上分配值。
*   `Rc<T>`: 引用计数，允许多重所有权。
*   `Ref<T>` 和 `RefMut<T>`: 通过 `RefCell<T>` 访问，在运行时强制借用规则。

## 1. Box<T>

`Box<T>` 是最简单的智能指针。它允许你将数据存储在堆上而不是栈上。栈上只保留指向堆数据的指针。

### 使用场景

1.  当类型的大小在编译时无法确定，但代码上下文需要确切大小时（例如递归类型）。
2.  当有大量数据，希望在转移所有权时不进行数据拷贝时。
3.  当希望拥有一个值并只关心它是否实现了特定 trait 而不是具体类型时（trait 对象）。

### 递归类型

Rust 需要在编译时知道类型占用多少空间。递归类型（如链表）理论上可以无限大。

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

## 2. 解引用 Trait (Deref Trait)

实现 `Deref` trait 允许我们自定义解引用运算符 `*` 的行为。

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

## 3. Drop Trait

`Drop` trait 允许我们在值离开作用域时执行代码（析构函数）。

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
```
