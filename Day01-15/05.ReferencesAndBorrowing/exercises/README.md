# Day 05 练习题 - 引用与借用

## 🎯 练习目标

通过这些练习,你将:
- 熟练区分和使用不可变引用 `&` 与可变引用 `&mut`
- 亲身体验 Rust 的借用检查器 (Borrow Checker) 规则
- 解决常见的数据竞争编译错误

## 📝 练习列表

### 练习 1: 借用检查器的愤怒 (基础)
**难度**: ⭐

**题目**:
下面的代码违反了借用规则。
请在**不删除**任何代码逻辑的前提下（不能直接删掉 `r1` 或 `r2`），通过调整代码顺序或作用域，使其编译通过。

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    r3.push_str(", world");
    println!("{}", r3);
}
```

**参考答案**: [查看答案](./solutions/exercise1.rs)

---

### 练习 2: 引用计数 (基础)
**难度**: ⭐⭐

**题目**:
编写一个函数 `word_count(s: &String) -> usize`。
它接收一个字符串的**不可变引用**，返回字符串中单词的数量（假设单词以空格分隔）。
在 `main` 中调用它。

**提示**:
- 使用 `s.split_whitespace()` 方法。

**参考答案**: [查看答案](./solutions/exercise2.rs)

---

### 练习 3: 修改内容 (进阶)
**难度**: ⭐⭐⭐

**题目**:
编写一个函数 `add_brackets(s: &mut String)`。
它接收一个字符串的**可变引用**，在字符串的首尾分别添加 `[` 和 `]`。

**示例**:
```rust
let mut s = String::from("Rust");
add_brackets(&mut s);
println!("{}", s); // 输出 "[Rust]"
```

**提示**:
- `s.insert_str(0, "[")`
- `s.push_str("]")`

**参考答案**: [查看答案](./solutions/exercise3.rs)

---

### 练习 4: 解引用 (进阶)
**难度**: ⭐⭐⭐

**题目**:
引用是指针。要修改引用指向的值（如果是简单类型如整数），通常需要 **解引用 (Dereference)**。
补全下面的函数，使其能正确将引用指向的整数加 1。

```rust
fn main() {
    let mut x = 10;
    increment(&mut x);
    println!("x is now {}", x); // 应该是 11
}

fn increment(n: &mut i32) {
    // 这里填什么？
}
```

**参考答案**: [查看答案](./solutions/exercise4.rs)

---

## 🏆 挑战任务

### 综合挑战: 悬垂引用的救赎
**难度**: ⭐⭐⭐⭐

**题目**:
下面的代码试图从函数返回一个引用，但是失败了（悬垂引用）。
请修复它。
修复的方法有两种：
1. 返回所有权 (String)。
2. 传入一个可变引用作为参数来修改外部变量。

请尝试**实现方法 2**。

```rust
// 错误代码
// fn main() {
//     let reference_to_nothing = dangle();
// }
//
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
```

**要求**:
修改 `dangle` 函数签名和实现，使其不返回引用，而是修改传入的参数。

**参考答案**: [查看答案](./solutions/challenge.rs)

## ✅ 完成检查清单

- [ ] 完成练习 1
- [ ] 完成练习 2
- [ ] 完成练习 3
- [ ] 完成练习 4
- [ ] 完成挑战任务

## 🎓 下一步

准备好了吗?让我们继续 [Day 06: 切片类型](../../06.Slices/README.md)!
