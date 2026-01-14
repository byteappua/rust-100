# Day 03: 函数与控制流

## 1. 函数 (Functions)

函数在 Rust 代码中非常普遍。你已经见过语言中最重要的函数之一：`main` 函数，它是许多程序的入口点。

### 定义函数

使用 `fn` 关键字定义函数。

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

### 参数 (Parameters)

Rust 是强类型语言，定义函数时必须指定参数的类型。

```rust
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
```

### 语句与表达式 (Statements and Expressions)

*   **语句**：执行操作但不返回值的指令。
*   **表达式**：计算并产生一个值。

Rust 是基于表达式的语言。注意表达式末尾没有分号。如果加了分号，它就变成了语句，不再返回值。

```rust
let y = {
    let x = 3;
    x + 1 // 这是一个表达式，返回 4
};
```

### 具有返回值的函数

函数可以向调用它的代码返回值。不需要命名返回值，但必须声明它们的类型。使用 `->` 箭头。

```rust
fn five() -> i32 {
    5 // 隐式返回
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

## 2. 控制流 (Control Flow)

### if 表达式

`if` 表达式允许根据条件执行不同的代码分支。

```rust
let number = 3;

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

因为 `if` 是一个表达式，我们可以要在 `let` 语句的右侧使用它：

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

### 循环 (Loops)

Rust 有三种循环：`loop`、`while` 和 `for`。

1.  **loop**: 无限循环，直到显式停止 (`break`)。

    ```rust
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // 可以返回值
        }
    };
    ```

2.  **while**: 条件循环。

    ```rust
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    ```

3.  **for**: 遍历集合。这是最常用的循环方式，因为安全且简洁。

    ```rust
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    // Range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    ```

## 3. 练习

1.  编写一个函数，将摄氏温度转换为华氏温度。
2.  生成第 n 个斐波那契数。
3.  打印圣诞颂歌 "The Twelve Days of Christmas" 的歌词，利用循环重复的部分。
