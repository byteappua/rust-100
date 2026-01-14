# Day 16: 闭包 (Closures)

闭包（Closures），也叫做匿名函数，是可以保存进变量或作为参数传递给其他函数的函数。与函数不同的是，闭包可以捕获调用者作用域中的值。

## 1. 定义闭包

闭包的定义语法：

```rust
let add_one = |x| x + 1;
let add_one_v2 = |x: i32| -> i32 { x + 1 };
```

*   `||` 包含参数。
*   可选的 `{}` 包含函数体（如果只有一行表达式可省略）。

## 2. 闭包类型推断

编译器通常可以推断参数和返回值的类型。

```rust
let example_closure = |x| x;
let s = example_closure(String::from("hello"));
// let n = example_closure(5); // 错误：类型已被锁定为 String
```

## 3. 捕获环境

闭包可以捕获其环境中的变量。

```rust
let x = 4;
let equal_to_x = |z| z == x;
let y = 4;
assert!(equal_to_x(y));
```

### 捕获方式 (Fn Traits)

闭包可以通过三种方式捕获环境，这对应于三个 Trait：

1.  **FnOnce** 消费从周围作用域捕获的变量（获取所有权）。闭包不能多次调用。
2.  **FnMut** 获取可变借用值。
3.  **Fn** 获取不可变借用值。

### `move` 关键字

强制闭包获取其使用环境值的所有权。

```rust
let x = vec![1, 2, 3];
let equal_to_x = move |z| z == x;
// println!("can't use x here: {:?}", x); // 错误：x 已移动
```

## 4. 练习：带有缓存的 Cacher

创建一个结构体，持有闭包和计算结果的缓存。

```rust
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
```
