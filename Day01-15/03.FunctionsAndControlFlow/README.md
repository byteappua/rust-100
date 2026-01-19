# Day 03: 函数与控制流 (Functions & Control Flow)

欢迎来到 **Rust 100 Days** 的第三天！今天我们将构建程序的骨架与肌肉。

在 Rust 中，函数不仅是逻辑的容器，更是所有权转移的通道；而控制流（Control Flow）则完全基于**表达式 (Expression)** 优先的哲学，这使得 Rust 的代码风格与其他语言（如 C/Java）有着微妙而重要的区别。

---

## 📋 目录 (Table of Contents)

1. [函数基础 (The Building Blocks)](#-函数基础-the-building-blocks)
2. [关键概念：语句与表达式 (Statements vs Expressions)](#-关键概念语句与表达式-statements-vs-expressions)
3. [控制流：条件与循环 (Logic & Loops)](#-控制流条件与循环-logic--loops)
4. [专业编码习惯 (Professional Habits)](#-专业编码习惯-professional-habits)
5. [练习与扩展](#-练习与扩展-exercises)

---

## 🧱 函数基础 (The Building Blocks)

函数是 Rust 代码的一等公民，使用 `fn` 关键字定义。

### 函数解剖学

```rust
// 关键字  函数名    参数(必须标注类型!)      返回类型
fn      add_one  (x: i32)               -> i32 {
    x + 1  // 函数体 (表达式)
}
```

* **参数强类型**: 必须显式声明每个参数的类型。
* **返回类型**: 使用 `->` 箭头声明。
* **隐式返回**: 函数体中**最后一行**如果没有分号，其值即为返回值。

> **⚠️ 注意**: Rust 不关心函数定义在调用之前还是之后，只要在同一个作用域内（或可见的模块中）即可。

---

## ⚖️ 关键概念：语句与表达式 (Statements vs Expressions)

这是 Rust 新手最容易困惑，也是 Rust 最"FP"（函数式编程）的地方。理解它，你才能写出地道的 Rust 代码。

* **语句 (Statement)**: 执行一个操作，**没有返回值**（或者说返回 Unit 类型 `()`）。
  * 标志：通常以 `;` 分号结尾。
  * 例子：`let y = 6;`, `println!("hi");`
* **表达式 (Expression)**: 计算并**产生一个值**。
  * 标志：通常**没有** `;` 分号结尾。
  * 例子：`5 + 6`, `{ let x = 3; x + 1 }`, `if condition { 5 } else { 6 }`

### 决策流程图

```mermaid
graph TD
    Start([代码片段]) --> HasSemi{结尾有分号吗<br>';' ?}
    HasSemi -- Yes --> Statement[语句 Statement]
    Statement --> ReturnsUnit[返回单元类型: ()]
    HasSemi -- No --> Expression[表达式 Expression]
    Expression --> ReturnsValue[返回计算出的值]
    
    style Statement fill:#f9f,stroke:#333
    style Expression fill:#9f9,stroke:#333
```

### 代码对比

```rust
fn main() {
    // 这是一个语句，因为有分号。y 的值是 ()
    // let x = (let y = 6); // ❌ 错误！语句不返回值，不能赋给 x
    
    // 这是一个块表达式 (Block Expression)
    let y = {
        let x = 3;
        x + 1 // ⬅️ 没有分号！这是一个表达式，返回 4
    }; // 分号结束 let 语句
    
    println!("y is: {}", y); // y is 4
}
```

---

## 🔀 控制流：条件与循环 (Logic & Loops)

### 1. `if` 表达式

在 Rust 中，`if` 是表达式，这意味着它可以**返回值**并赋值给变量。

```rust
let condition = true;
// if 和 else 分支的返回类型必须一致！
let number = if condition { 5 } else { 6 };
```

### 2. 循环体系 (Loop System)

Rust 提供了三种循环工具，分别应对不同场景。

```mermaid
graph TD
    Start([开始循环]) --> KnownRange{是遍历范围<br>或集合吗?}
    KnownRange -- Yes --> For[使用 for 循环<br>(最安全/最常用)]
    KnownRange -- No --> InfLoop{需要无限循环<br>或手动控制退出?}
    InfLoop -- Yes --> Loop[使用 loop 循环<br>(可返回值)]
    InfLoop -- No --> While[使用 while 循环<br>(基于条件)]
    
    style For fill:#cfc,stroke:#333,stroke-width:2px
```

#### A. `for` 循环 (最推荐)

最安全、最简洁。用于遍历集合或 `Range`。

```rust
// 1..4 是 Range 类型 (1, 2, 3)
for number in 1..4 {
    println!("{}!", number);
}

// 遍历数组
let a = [10, 20, 30];
for element in a {
    println!("Value: {}", element);
}
```

#### B. `loop` 循环

无限循环，直到显式 `break`。它有一个绝技：**`break` 可以带返回值**。

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // 返回 20
    }
};
```

#### C. `while` 循环

当条件为真时运行。容易出现索引越界问题，通常被 `for` 取代。

---

## 🧢 专业编码习惯 (Professional Habits)

1. **省略分号**：在函数末尾或块末尾返回时，习惯省略 `return` 和 `;`，直接写表达式。
    * ✅ `x + 1`
    * ❌ `return x + 1;` (除非是提前返回)
2. **避免 `while` 遍历**：不仅代码长，而且编译器难以优化边界检查。始终优先使用 `for`。
3. **Range 语法**：
    * `0..5` : 左闭右开 `[0, 5)` (包含0, 不含5)
    * `0..=5`: 全闭 `[0, 5]` (包含0, 包含5)
4. **Early Return**: 为了减少嵌套层级，使用 `return;` 提前退出函数。

```rust
fn process(x: i32) {
    if x < 0 { return; } // Early return
    // ... 主逻辑
}
```

---

## 🏋️ 练习与扩展 (Exercises)

光看不练假把式。

1. **斐波那契生成器**: 编写一个函数 `fib(n: u32) -> u32`，生成第 n 个斐波那契数。
2. **温度转换器**: 编写函数实现华氏度和摄氏度的双向转换。
3. **圣诞歌**: 利用 `for` 循环打印 "The Twelve Days of Christmas" 的歌词（利用数组存储重复部分）。

👉 **[点击这里访问详细练习题目录](./exercises/README.md)**

---

## ⏭️ 下一步

你现在已经掌握了 Rust 的语法基础。但要真正理解 Rust 的精髓，我们需要挑战 Boss 级的概念。下一章，我们将面对 Rust 最核心、最独特的特性。

[**Day 04: 所有权 (Ownership)**](../04.Ownership/README.md)
