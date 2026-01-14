# Day 21: 引用循环与内存泄漏

Rust 的内存安全性保证使其很难发生内存泄漏，但不是不可能。如果使用 `Rc<T>` 和 `RefCell<T>` 创建了引用循环，items 的引用计数永远不会降为 0，内存也就永远不会被释放。

## 1. 制造引用循环

如果两个 `Rc` 指针相互引用，就会形成循环。

```rust
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // 取消下面的注释会造成栈溢出，因为 debug 打印会尝试递归打印循环引用
    // println!("a next item = {:?}", a.tail());
}
```

## 2. 使用 Weak<T> 避免引用循环

`Weak<T>` 是弱引用，不增加 `strong_count`，而是增加 `weak_count`。当 `strong_count` 为 0 时，即使 `weak_count` 不为 0，对象也会被清理。

你需要使用 `upgrade` 方法将 `Weak<T>` 升级为 `Option<Rc<T>>` 才能访问数据，这确保了数据存在。

### 示例：树形结构（父节点指针）

子节点拥有父节点的引用，如果用 `Rc` 就会造成循环。应该使用 `Weak`。

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

## 3. Strong vs Weak

*   `Rc::clone` 增加 `strong_count`。`Rc<T>` 实例在 `strong_count` 为 0 时被清理。
*   `Rc::downgrade` 增加 `weak_count`。`Weak<T>` 不影响清理时间。
*   `Weak<T>` 用于打破循环引用，防止内存泄漏。
