# Day 15: 自动化测试 (Automated Testing)

Rust 是一门非常重视正确性的语言，它在语言本身包含了对自动化测试的一流支持。

## 1. 单元测试 (Unit Tests)

单元测试主要是在**模块内部**进行，用于测试独立的功能模块（函数、方法等）。它可以测试私有接口。

### 编写单元测试

通常在每个源代码文件的底部创建一个 `tests` 模块，并使用 `#[cfg(test)]` 标注。

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*; // 引入父模块的所有内容

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_internal() {
        // 可以测试私有函数
        assert_eq!(internal_adder(1, 2), 3);
    }
}
```

### 常用断言宏

*   `assert!(expression)`: 断言表达式为 true。
*   `assert_eq!(left, right)`: 断言两个值相等（需要实现 `PartialEq` 和 `Debug`）。
*   `assert_ne!(left, right)`: 断言两个值不相等。

### 使用 Result<T, E> 编写测试

测试函数也可以返回 `Result`。如果返回 `Err`，测试失败。

```rust
#[test]
fn it_works_with_result() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

## 2. 处理 Panic

### should_panic 属性

检查代码是否按预期 panic。

```rust
#[test]
#[should_panic(expected = "Guess value must be between 1 and 100")] // 可选：检查 panic 信息
fn greater_than_100() {
    Guess::new(200);
}
```

## 3. 运行测试

```bash
cargo test
```

Rust 默认并行运行测试。

### 常用参数

*   `cargo test -- --test-threads=1`: 单线程运行（避免并行干扰，如数据库测试）。
*   `cargo test -- --show-output`: 显示成功测试的标准输出（默认会被捕获并隐藏）。
*   `cargo test <test_name>`: 运行指定名称的测试（支持模糊匹配）。

### 忽略测试

对于耗时较长的测试，可以使用 `#[ignore]` 标记。

```rust
#[test]
#[ignore]
fn expensive_test() {
    // ...
}
```

运行被忽略的测试：

```bash
cargo test -- --ignored
```

## 4. 集成测试 (Integration Tests)

集成测试位于项目根目录下的 `tests` 目录中。它们被视为完全独立的 crate，只能访问你的库的**公有 API**。

**目录结构**:
```text
my_crate
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

**tests/integration_test.rs**:
```rust
use my_crate; // 必须像外部用户一样导入

#[test]
fn it_adds_two() {
    assert_eq!(4, my_crate::add_two(2));
}
```

无需 `#[cfg(test)]`，因为 `tests` 目录只在测试时编译。

## 5. 文档测试 (Doc-tests)

Rust 允许在文档注释中编写测试。这确保了你的文档示例永远是可运行的！

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

运行 `cargo test` 时也会运行文档测试。
注意：文档测试只对库 crate (library crate) 有效。
