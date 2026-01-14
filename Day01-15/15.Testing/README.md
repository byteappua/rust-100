# Day 15: 自动化测试 (Automated Testing)

Rust 的测试功能由 `test` 属性、`cargo test` 命令和 `should_panic` 属性等组成。

## 1. 编写测试

测试通常放在 `src` 目录下的源文件中，或者放在 `tests` 目录下的集成测试文件中。

### 单元测试

在模块内定义测试，使用 `#[cfg(test)]` 标注模块，使用 `#[test]` 标注测试函数。

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

### 常用宏

*   `assert!(expression)`: 断言表达式为 true。
*   `assert_eq!(left, right)`: 断言两个值相等。
*   `assert_ne!(left, right)`: 断言两个值不相等。
*   `panic!("message")`: 导致测试失败。

## 2. 测试 Panic

使用 `#[should_panic]` 属性检查代码是否按预期 panic。

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

## 3. 运行测试

```bash
cargo test
```

*   `cargo test --help`: 查看帮助。
*   `cargo test -- --test-threads=1`: 单线程运行（避免并行干扰）。
*   `cargo test -- --show-output`: 显示成功测试的输出。

## 4. 集成测试

集成测试完全位于你的库之外。它们与其他外部代码一样使用你的库。创建 `tests` 目录。

**tests/integration_test.rs**:
```rust
use testing_demo;

#[test]
fn it_adds_two() {
    assert_eq!(4, testing_demo::add_two(2));
}
```
