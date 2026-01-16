# Day 05: 引用与借用 (References and Borrowing)

## 📝 学习目标
- 理解引用的概念 (Pointer without ownership)
- 掌握 **借用 (Borrowing)** 的规则
- 区分 **不可变引用** (`&T`) 和 **可变引用** (`&mut T`)
- 熟练处理引用作用域和数据竞争问题
- 了解悬垂引用 (Dangling References) 及其预防

## 🎯 为什么要学这个
所有权系统虽然安全，但如果每次使用数据都要转移所有权再还回来（像 Day 04 练习那样），代码会非常啰嗦。
**引用**允许我们使用值而不获取其所有权。这就像是现实生活中的"借用"：你借了一本书看，看完得还回去，你不能把书烧了（Drop）或者转手卖给别人（Move）。

## 📖 核心概念

### 1. 引用 (Reference)
引用是指向内存中某处数据的指针，但它保证总是有效的。
- 符号：`&`
- 创建引用称为 **借用**。

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1); // &s1 创建了一个引用
// s1 仍然有效
```

### 2. 可变引用 (Mutable References)
默认引用是不可变的。如果你想修改借来的数据，必须使用可变引用。
- 符号：`&mut`

```rust
let mut s = String::from("hello");
change(&mut s); // 传递可变引用
```

### 3. 借用规则 (The Rules of Borrowing)
这是 Rust 编译器 (Borrow Checker) 最严格检查的地方：

1. **要么** 只能有一个可变引用，**要么** 只能有任意数量的不可变引用。
   - 这被称为 "Reader-Writer Lock" 模式：多读单写。
2. 引用必须总是有效的 (不能指向空或无效内存)。

### 4. 数据竞争 (Data Race)
Rust 的规则在编译期就根除了数据竞争。数据竞争发生于：
- 两个或更多指针同时访问同一数据。
- 至少有一个指针被用来写入数据。
- 没有同步数据的机制。

Rust 拒绝编译存在数据竞争的代码：
```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // 错误！同时存在两个可变引用
```

### 5. 悬垂引用 (Dangling References)
在 C++ 中，你可能返回一个指向栈上局部变量的指针，函数结束后该变量被释放，指针变成了"悬垂指针"。
Rust 编译器会保证这不可能发生：引用（借用）的生命周期不能超过数据所有者的生命周期。

## 💻 代码示例

### 示例 1: 不可变借用
```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 借用 s1
    println!("The length of '{}' is {}.", s1, len); // s1 依然可用
}

fn calculate_length(s: &String) -> usize { // s 是引用，不拥有所有权
    s.len()
} // s 离开作用域，什么也不会发生（不会 drop 堆数据）
```

### 示例 2: 可变借用
```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s); // 输出 "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### 示例 3: 作用域与引用的生命周期
注意：引用的作用域从声明开始，到**最后一次使用**结束。这意味着编译器比以前更聪明了 (NLL - Non-Lexical Lifetimes)。

```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
println!("{} and {}", r1, r2);
// r1 和 r2 在这里不再使用了

let r3 = &mut s; // 没问题！因为 r1, r2 已经结束使用
println!("{}", r3);
```

## 🏋️ 练习题

- **练习 1**: 修复借用检查器错误
- **练习 2**: 使用可变引用修改数据
- **练习 3**: 引用混合使用的问题

👉 **[点击这里查看练习题](./exercises/README.md)**

## 🤔 常见问题 (FAQ)

### Q1: 为什么要限制同时只能有一个可变引用？
A: 为了防止数据竞争。如果两个指针都能写，谁先谁后？如果一个在读一个在写，读到的是写了一半的脏数据怎么办？Rust 通过强制"多读单写"在编译期解决了这个问题。

### Q2: `*` 符号是做什么的？
A: `*` 是 **解引用 (Dereference)** 操作符，是 `&` 的逆操作。通过引用访问数据通常是自动解引用的（比如调用方法时），但在某些情况下（如修改数值）你需要显式使用 `*r = 10`。

## 💡 最佳实践
- **能用不可变引用就别用可变引用**：这让代码更清晰，编译器优化空间更大。
- **利用 NLL 特性**：如果你需要再次借用，确保之前的引用已经不再使用了。
- **避免在长函数中持有可变引用**：这会锁定对象，阻止其他地方读取。

## 🔗 扩展阅读
- [Rust 程序设计语言 - 引用与借用](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## 📚 本节要点回顾
- `&T` 是只读引用，`&mut T` 是读写引用。
- 借用规则：1个可变 OR N个不可变。
- 引用就像借书，必须有借有还，且不能借已销毁的书。

## ⏭️ 下一步
引用是指向整个数据的。如果我们只想引用数据的一部分（比如字符串的一个片段）呢？

下一节: [Day 06: 切片类型](../06.Slices/README.md)
