# Day 08 练习题 - 枚举与模式匹配

## 🎯 练习目标

通过这些练习,你将:
- 掌握枚举 (`Enum`) 的定义和使用。
- 掌握 `Option<T>` 枚举处理空值。
- 掌握 `match` 控制流运算符。
- 理解 `match` 的穷尽性检查。

## 📝 练习列表

### 练习 1: 定义消息枚举 (基础)
**难度**: ⭐

**题目**:
定义一个名为 `Message` 的枚举，包含以下变体：
- `Quit` (无数据)
- `Move` (包含匿名结构体 `{ x: i32, y: i32 }`)
- `Write` (包含 `String`)
- `ChangeColor` (包含三个 `i32`)

在 `main` 函数中创建这些变体的实例，并打印它们（提示：加上 `#[derive(Debug)]`）。

**参考答案**: [查看答案](./solutions/exercise1.rs)

---

### 练习 2: Option 的加法 (基础)
**难度**: ⭐⭐

**题目**:
编写一个函数 `plus_one`，它接收一个 `Option<i32>`。
- 如果输入是 `Some(i)`，返回 `Some(i + 1)`。
- 如果输入是 `None`，返回 `None`。

**核心考点**: `match` 表达式的使用。

**参考答案**: [查看答案](./solutions/exercise2.rs)

---

### 练习 3: 硬币分类器 (基础)
**难度**: ⭐⭐

**题目**:
定义一个枚举 `Coin` (Penny, Nickel, Dime, Quarter)。
编写一个函数 `value_in_cents`，接收 `Coin` 并返回其面值（整数）。
- Penny: 1
- Nickel: 5
- Dime: 10
- Quarter: 25

**参考答案**: [查看答案](./solutions/exercise3.rs)

---

## 🏆 挑战任务

### 思考题: if let 语法
当你只关心一种情况而忽略其他所有情况时，`match` 会显得很冗长：
```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
```
请尝试使用 `if let` 重写上面的代码。

**提示**:
```rust
if let Some(max) = config_max {
    // ...
}
```

---

## ✅ 完成检查清单

- [ ] 完成练习 1
- [ ] 完成练习 2
- [ ] 完成练习 3
- [ ] 掌握 if let 写法

## 🎓 下一步

准备好了吗?让我们继续 [Day 09: 模块系统](../../09.Modules/README.md)!
