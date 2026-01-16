# Day 04 练习题 - 所有权

## 🎯 练习目标

通过这些练习,你将:
- 深刻理解所有权的 **Move** 语义
- 学会使用 `.clone()` 解决所有权转移问题
- 区分 **Copy** 类型和非 Copy 类型

## 📝 练习列表

### 练习 1: 被移动的字符串 (基础)
**难度**: ⭐

**题目**:
下面的代码无法编译。请解释原因，并修复它，使得 `s1` 在赋值给 `s2` 后仍然可以使用。
（有两种修复方法：Clone 或者使用引用。这里请使用 Clone）。

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

**参考答案**: [查看答案](./solutions/exercise1.rs)

---

### 练习 2: 函数所有权陷阱 (基础)
**难度**: ⭐⭐

**题目**:
下面的代码中，`s` 在传递给 `takes_ownership` 后就不能用了。
请修改代码，让 `main` 函数在调用 `takes_ownership` 后还能打印 `s`。
**限制**: 你不能修改 `takes_ownership` 的签名（不能改为接收引用）。你只能修改 `main` 函数。

```rust
fn main() {
    let s = String::from("Rust");
    takes_ownership(s);
    println!("I still have: {}", s); // 这里报错
}

fn takes_ownership(some_string: String) {
    println!("I took: {}", some_string);
}
```

**参考答案**: [查看答案](./solutions/exercise2.rs)

---

### 练习 3: 归还所有权 (进阶)
**难度**: ⭐⭐⭐

**题目**:
有时候我们需要把所有权拿回来。
编写一个函数 `take_and_return(s: String) -> String`。
它接收一个字符串，在原来字符串后面追加 " World"，然后返回新的字符串（归还所有权）。

**示例**:
```rust
let s1 = String::from("Hello");
let s2 = take_and_return(s1);
println!("{}", s2); // 输出 "Hello World"
// println!("{}", s1); // s1 应该无效
```

**参考答案**: [查看答案](./solutions/exercise3.rs)

---

### 练习 4: Copy vs Move (进阶)
**难度**: ⭐⭐⭐

**题目**:
下面的元组 `t` 包含一个 `i32` 和一个 `String`。
当我们把 `t` 赋值给 `t2` 时，发生了什么？
尝试分别打印 `t.0` 和 `t.1`，看看哪些部分还能访问，哪些不能。

```rust
fn main() {
    let t = (1, String::from("hello"));
    let t2 = t;

    // 尝试取消注释下面这行
    // println!("{:?}", t);
}
```

**挑战**: 如果元组是 `(i32, i32)` 呢？尝试验证它是否实现了 Copy。

**参考答案**: [查看答案](./solutions/exercise4.rs)

---

## 🏆 挑战任务

### 综合挑战: 堆内存模拟
**难度**: ⭐⭐⭐⭐

**题目**:
为了加深对 Stack 和 Heap 的理解，请编写程序打印出不同变量的内存地址（指针地址）。

1. 创建一个字符串 `s = String::from("hello")`。
2. 打印 `s` 在栈上的地址 (`&s`)。
3. 打印 `s` 指向的堆上的数据的地址 (`s.as_ptr()`)。
4. 创建 `let s2 = s`。
5. 打印 `s2` 在栈上的地址。
6. 打印 `s2` 指向的堆上的数据的地址。

**观察**:
- `s` 和 `s2` 的栈地址一样吗？
- `s` 和 `s2` 指向的堆地址一样吗？

这能直观地证明 Move 只是拷贝了栈上的指针，而没有拷贝堆数据。

**参考答案**: [查看答案](./solutions/challenge.rs)

## ✅ 完成检查清单

- [ ] 完成练习 1
- [ ] 完成练习 2
- [ ] 完成练习 3
- [ ] 完成练习 4
- [ ] 完成挑战任务

## 🎓 下一步

准备好了吗?让我们继续 [Day 05: 引用与借用](../../05.ReferencesAndBorrowing/README.md)!
