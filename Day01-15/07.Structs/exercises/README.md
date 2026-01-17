# Day 07 练习题 - 结构体 (Structs)

## 🎯 练习目标

通过这些练习,你将:
- 掌握结构体的定义与实例化。
- 掌握结构体更新语法 (`Struct Update Syntax`)。
- 掌握方法的定义与调用 (`impl` 块)。
- 理解结构体字段的所有权。

## 📝 练习列表

### 练习 1: 定义你的书库 (基础)
**难度**: ⭐

**题目**:
1. 定义一个名为 `Book` 的结构体，包含 `title` (String), `author` (String), `pages` (u32)。
2. 在 `main` 函数中实例化一本具体的书。
3. 打印出这本书的各个字段。

**参考答案**: [查看答案](./solutions/exercise1.rs)

---

### 练习 2: 用户系统更新 (基础)
**难度**: ⭐

**题目**:
给定一个现有的 `User` 实例：
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```
请使用 **结构体更新语法** 创建一个新的 `user2`：
- `email` 改为 "another@example.com"
- 其他字段与 `user1` 相同

**注意**: 思考一下，创建 `user2` 后，`user1` 还能使用吗？为什么？

**参考答案**: [查看答案](./solutions/exercise2.rs)

---

### 练习 3: 矩形计算 (进阶)
**难度**: ⭐⭐

**题目**:
1. 定义 `Rectangle` 结构体（宽、高）。
2. 为它实现两个方法：
   - `area(&self) -> u32`: 计算面积。
   - `can_hold(&self, other: &Rectangle) -> bool`: 判断当前矩形是否能包含另一个矩形（宽和高都更大）。
3. 编写测试代码验证你的方法。

**参考答案**: [查看答案](./solutions/exercise3.rs)

---

## 🏆 挑战任务

### 思考题: 为什么结构体中通常用 String 而不是 &str?
```rust
struct User {
    username: &str, // 编译报错
    // ...
}
```
**原因**: 结构体实例通常需要拥有其数据。如果使用引用 `&str`，结构体实例的生命周期就不能超过引用的数据的生命周期。这需要引入 **生命周期标注 (Lifetime Annotations)** (Day 14 会讲)。
**目前建议**: 在结构体中尽量使用 `String`，除非你明确知道你在做什么。

---

## ✅ 完成检查清单

- [ ] 完成练习 1
- [ ] 完成练习 2
- [ ] 完成练习 3
- [ ] 理解为什么 user1 可能部分失效

## 🎓 下一步

准备好了吗?让我们继续 [Day 08: 枚举与模式匹配](../../08.Enums/README.md)!
