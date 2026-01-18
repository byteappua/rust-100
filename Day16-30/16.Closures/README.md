# Day 16: 闭包 (Closures)

## 📝 学习目标
- 理解闭包的基本概念和语法。
- 掌握闭包如何捕获环境（`Fn`, `FnMut`, `FnOnce`）。
- 学会使用 `move` 关键字强制转移所有权。
- 能够将闭包作为参数传递或作为返回值。

## 🎯 为什么要学这个
闭包是函数式编程的核心概念之一，在 Rust 中被广泛用于：
- **迭代器适配器**：如 `map`, `filter` 等。
- **线程生成**：`thread::spawn` 接受一个闭包作为新线程的任务。
- **回调函数**：在异步编程或事件驱动编程中。
- **延迟计算**：仅在需要时执行代码。

相比普通函数，闭包最强大的能力是**捕获其定义作用域中的变量**，这使得它们非常灵活。

## 📖 核心概念

### 1. 定义闭包
闭包通常不需要像函数那样标注参数和返回值类型，编译器能自动推断。

```rust
let add_one = |x| x + 1;
// 显式标注类型
let add_one_explicit = |x: i32| -> i32 { x + 1 };
```

### 2. 捕获环境与 Trait
闭包根据其如何使用捕获的变量，自动实现以下 Trait 之一（或多个）：

*   **`FnOnce`**：闭包消耗捕获的变量（获取所有权）。只能调用一次。
*   **`FnMut`**：闭包可变地借用捕获的变量。可以多次调用，且可能会改变环境。
*   **`Fn`**：闭包不可变地借用捕获的变量。可以多次调用，不改变环境。

### 3. `move` 关键字
默认情况下，闭包尽可能以引用的方式捕获变量。使用 `move` 关键字可以强制闭包获取捕获变量的所有权。这在多线程场景中非常重要。

```rust
let list = vec![1, 2, 3];
// 强制获取 list 的所有权，否则 list 的生命周期可能短于新线程
std::thread::spawn(move || {
    println!("From thread: {:?}", list);
});
```

## 💻 代码示例

### 示例 1: 基础用法与类型推断

```rust
fn main() {
    let example_closure = |x| x;

    // 第一次调用锁定了类型为 String
    let s = example_closure(String::from("hello"));
    println!("s: {}", s);

    // let n = example_closure(5); // 编译错误！类型已推断为 String
}
```

### 示例 2: 带有记忆化 (Memoization) 的 Cacher

我们可以创建一个结构体，它持有一个闭包，并缓存该闭包的结果。

```rust
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
```

## 🏋️ 练习题

### 练习 1: 基础闭包
编写一个闭包，接受一个字符串，返回该字符串的大写形式。
[查看参考答案](./exercises/solutions/exercise1_solution.rs)

### 练习 2: 捕获计数器
实现一个闭包，每次调用时使内部捕获的计数器加 1，并返回当前值。思考需要使用哪种 Fn Trait？
[查看参考答案](./exercises/solutions/exercise2_solution.rs)

### 练习 3: 泛型 Cacher
实现一个可以处理不同类型参数和返回值的泛型 `Cacher` 结构体。它应该在第一次调用 `value` 方法时执行闭包并缓存结果，后续调用直接返回缓存。
[查看参考答案](./exercises/solutions/exercise3_solution.rs)

## 🤔 常见问题

### Q1: 什么时候该用 `move`？
A: 当你需要将闭包传递给新线程，或者希望闭包完全拥有它捕获的数据（例如为了防止外部修改，或者为了返回闭包）时。

### Q2: 为什么不能直接返回闭包？
A: 闭包没有具体的大小（Unsized），因为每个闭包都是独特的匿名类型。要返回闭包，通常需要将其装箱：`Box<dyn Fn(i32) -> i32>` 或使用 `impl Trait` 语法：`fn return_closure() -> impl Fn(i32) -> i32`。

## 🔗 扩展阅读
- [Rust Book: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Rust by Example: Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)
