# Day 05: 引用与借用 (References and Borrowing)

如果我们想使用变量的值但不获取其所有权，可以使用 **引用 (Reference)**。获取引用作为函数参数称为 **借用 (Borrowing)**。

## 1. 引用 (Reference)

像 `&String` 这样的类型就是引用。它们允许你使用值但不获取所有权。

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传递引用
    println!("The length of '{}' is {}.", s1, len); // s1 仍然有效
}

fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // s 离开作用域，因为它不拥有所有权，所以什么也不会发生
```

## 2. 可变引用 (Mutable References)

默认情况下，引用是不可变的。如果我们想修改借用的值，需要使用 **可变引用**。

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### 限制

1.  **特定作用域内，对特定数据只能有一个可变引用**。这个限制允许 Rust 在编译时就避免数据竞争 (Data Races)。
2.  **不能同时拥有可变引用和不可变引用**。

```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
let r3 = &mut s; // 大问题！
// println!("{}, {}, and {}", r1, r2, r3);
```

注意：引用的作用域从声明开始，一直持续到最后一次使用为止。

## 3. 悬垂引用 (Dangling References)

在具有指针的语言中，很容易产生悬垂指针（指向已被释放内存的指针）。Rust 编译器保证引用永远不会是悬垂引用：如果拥有某个数据的引用，编译器确保数据在引用结束前不会失效。

```rust
fn main() {
    // let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回字符串 s 的引用
// } // s 离开作用域并被丢弃。错误！
```

解决方法是直接返回 String。

## 4. 规则总结

1.  在任意给定时间，**要么** 只能有一个可变引用，**要么** 只能有多个不可变引用。
2.  引用必须总是有效的。
