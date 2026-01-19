# Day 12 练习题

## 练习 1: 泛型容器

文件: `src/exercise1.rs`

定义一个泛型结构体 `Container<T>`，它持有一个值 `value: T`。
为它实现 `new` 方法和 `get_value` 方法。

## 练习 2: 泛型交换函数

文件: `src/exercise2.rs`

编写一个泛型函数 `swap_values`，它接受两个相同类型的参数，并返回交换后的元组。
虽然 `std::mem::swap` 可以原地交换，但这里我们练习通过返回值交换：`fn swap_values<T>(a: T, b: T) -> (T, T)`。

## 练习 3: 混合泛型

文件: `src/exercise3.rs`

定义 `Point<X1, Y1>` 和 `Point<X2, Y2>`。
实现一个方法 `mixup`，它接受另一个 `Point`，并返回一个新的 `Point`，其 x 来自 `self`，y 来自 `other`。
注意：新 Point 的类型可能与前两个都不同！

```rust
let p1 = Point { x: 5, y: 10.4 };
let p2 = Point { x: "Hello", y: 'c' };
let p3 = p1.mixup(p2);
// p3.x should be 5, p3.y should be 'c'
```

## ✅ 验证你的答案

```bash
rustc src/exercise1.rs && ./exercise1
rustc src/exercise2.rs && ./exercise2
rustc src/exercise3.rs && ./exercise3
```
