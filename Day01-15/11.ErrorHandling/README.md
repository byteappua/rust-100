# Day 11: 错误处理 (Error Handling)

## 📝 学习目标

- 理解 Rust **可恢复 (Result)** 与 **不可恢复 (Panic)** 错误的区别
- 掌握 **`Result<T, E>`** 的核心用法
- 熟练使用 **`?` 运算符** 传播错误
- 学会 **`unwrap`** 和 **`expect`** 的适用场景
- 了解如何在 `main` 函数中优雅地处理错误

## 🎯 核心哲学：向左走？向右走？

Rust 将错误分为两大类。在写代码时，你首先要面对的决策是：

```mermaid
graph TD
    Start[错误发生] --> Q1{问题严重吗?}
    Q1 --"Yes (Bug/不可恢复)"--> Panic[Panic! (崩溃退出)]
    Q1 --"No (预期情况/可恢复)"--> Result[Result (返回错误值)]
    
    Panic --> Ex1[数组越界]
    Panic --> Ex2[逻辑不可能到达的分支]
    
    Result --> Ex3[文件未找到]
    Result --> Ex4[网络超时]
    Result --> Ex5[解析数字失败]
    
    style Panic fill:#ffcccc,stroke:#ff0000
    style Result fill:#ccffcc,stroke:#00ff00
```

1. **不可恢复错误 (panic!)**: 表示代码中有 Bug。Rust 会打印错误信息，清理栈内存，然后退出。
2. **可恢复错误 (Result)**: 表示即便代码没写错也可能发生的情况（如文件不存在）。你需要显式处理它。

---

## 💥 不可恢复错误：`panic!`

当程序遇到无法处理的糟糕情况时，就 panic。

```rust
fn main() {
    // 显式 panic
    panic!("crash and burn");
    
    // 隐式 panic (例如数组越界)
    let v = vec![1, 2, 3];
    v[99];
}
```

---

## 🛡️ 可恢复错误：`Result<T, E>`

`Result` 是一个枚举，定义如下：

```rust
enum Result<T, E> {
    Ok(T),  // 成功，包含值 T
    Err(E), // 失败，包含错误 E
}
```

### 处理 Result 的四种姿势

| 方法 | 说明 | 适用场景 |
| :--- | :--- | :--- |
| **`match`** | 显式处理 `Ok` 和 `Err` 分支 | 需要对错误做精细处理（如文件不存在则创建） |
| **`unwrap()`** | 成功返值，失败 panic | 原型开发、测试、你 100% 确定不会失败时 |
| **`expect(msg)`** | 同 `unwrap`，但可自定义 panic 信息 | 同上，但比 unwrap 提供更多调试信息 (推荐) |
| **`?`** | 成功返值，失败则 **立即返回 Err 给调用者** | 函数内部，想把锅甩给调用者时 (最常用) |

---

## 🚀 传播错误：`?` 运算符

`?` 是 Rust 中极其优雅的语法糖。它能消除大量的 `match` 嵌套。

### `?` 的工作流程

```mermaid
graph LR
    Step1[代码: File::open?] --> Check{是 Ok 还是 Err?}
    Check --Ok(file)--> ReturnVal[表达式求值为 file] --> Continue[继续执行下一行]
    Check --Err(e)--> ReturnErr[return Err(e)] --> Exit[函数立即结束]
    
    style ReturnErr fill:#ffcccc,stroke:#ff0000
    style Continue fill:#ccffcc,stroke:#00ff00
```

### 代码对比

**未使用 `?` (啰嗦)**:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // 手动返回错误
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), // 手动返回错误
    }
}
```

**使用 `?` (优雅)**:

```rust
fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    // 链式调用：打开文件 -> 读取内容
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

---

## 💻 实用示例

### 示例 1: `main` 函数也返回 Result

为了在 `main` 中使用 `?`，我们需要修改它的返回类型。

```rust
use std::fs::File;
use std::error::Error;

// Box<dyn Error> 意味着 "任意类型的错误"
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?; // 如果失败，程序以非0状态码退出
    
    Ok(())
}
```

### 示例 2: 自定义验证错误

```rust
#[derive(Debug)]
struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            // 这里用 panic 是合适的，因为违反了 API 契约（Bug）
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}
```

---

## 🏋️ 练习题

👉 **[点击这里查看练习题](./exercises/README.md)**

1. **安全除法**: 编写一个除法函数，当除数为 0 时返回 Err。
2. **文件读取**: 使用 `?` 重构文件读取代码。
3. **解析 CSV**: 尝试解析一个可能格式错误的字符串，处理解析错误。

---

## 💡 最佳实践

1. **库代码优先用 Result**: 编写给别人用的库时，尽量不要 panic，把决定权交给用户。
2. **善用 `expect`**: 除非写 Demo，否则尽量用 `expect` 代替 `unwrap`，这能让你在 Debug 时省下很多头发。
3. **转换错误**: 有时这层函数返回 `AppError`，下层返回 `io::Error`。可以使用 `map_err` 进行转换。

---

## ⏭️ 下一步

在前面的课程中，我们为了复用代码，比如求两个 `i32` 的最大值，可能写了 `fn max(a: i32, b: i32)`。如果要处理 `f64` 怎么办？再写一个？
不，Rust 有 **泛型 (Generics)**。

下一节: [Day 12: 泛型 (Generics)](../12.Generics/README.md)
