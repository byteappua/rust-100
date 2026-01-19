# Day 15: 自动化测试 (Automated Testing)

## 📝 学习目标
- 掌握 Rust 单元测试的编写 (`#[test]`)
- 熟练使用断言宏 (`assert!`, `assert_eq!`, `assert_ne!`)
- 掌握如何测试 `panic!` (`should_panic`)
- 了解集成测试与文档测试
- 掌握 `cargo test` 的常用参数

## 🎯 为什么要学这个
- **正确性**: 确保代码按预期工作。
- **重构信心**: 当你修改代码时，测试能立刻告诉你是否破坏了现有功能。
- **文档**: 测试用例本身就是很好的代码使用示例（特别是文档测试）。
- **Rust 原生支持**: 可以在代码文件中直接写测试，不需要额外的框架。

## 📖 核心概念

### 1. 单元测试 (Unit Tests)
通常位于源码文件的底部，封装在 `mod tests` 模块中。
- 测试私有接口。
- 使用 `#[cfg(test)]` 标注，确保只在测试时编译。

```rust
fn internal_add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_add() {
        assert_eq!(internal_add(2, 2), 4);
    }
}
```

### 2. 集成测试 (Integration Tests)
位于项目根目录的 `tests` 目录下。
- 只能调用公有 API (就像外部用户一样)。
- 专门用于测试多个模块间的交互。

### 3. 文档测试 (Doc-tests)
写在文档注释 `///` 中的代码块。`cargo test` 会自动运行它们。
这确保了你的文档示例永远是可运行的，不会过期。

### 4. 常用断言
- `assert!(expr)`: 验证表达式为真。
- `assert_eq!(left, right)`: 验证相等。
- `assert_ne!(left, right)`: 验证不等。
- `debug_assert!(...)`: 只在 Debug 模式下运行。

## 💻 代码示例

### 示例 1: 基本测试与 Panic 测试

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
    #[should_panic(expected = "must be between 1 and 100")] // 检查 panic 信息包含特定字符串
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

### 示例 2: 使用 Result 编写测试
允许在测试中使用 `?` 运算符。

```rust
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

## 🏋️ 练习题

我们准备了练习题来帮助你掌握测试的编写。

- **练习 1**: 编写基本的单元测试
- **练习 2**: 测试自定义结构体与 panic
- **练习 3**: 编写文档测试（模拟）

👉 **[点击这里查看练习题](./exercises/README.md)**

## 🤔 常见问题 (FAQ)

### Q1: `cargo test` 默认是并行运行的吗？
A: 是的。如果你的测试相互依赖（例如都操作同一个文件或数据库），这可能会导致问题。可以使用 `cargo test -- --test-threads=1` 来串行运行测试。

### Q2: 既然可以测试私有函数，那还需要集成测试吗？
A: 需要。单元测试关注实现的细节（白盒），集成测试关注对外暴露的 API 是否好用、多个组件能否正确协作（黑盒）。两者缺一不可。

## 💡 最佳实践
- **TDD (测试驱动开发)**: 先写一个失败的测试，再写代码让它通过。
- **测试覆盖率**: 尽量覆盖各种边缘情况 (Edge Cases)，不仅仅是正常路径 (Happy Path)。
- **文档即测试**: 尽量为公有 API 编写包含示例代码的文档注释。

## 🔗 扩展阅读
- [Rust 程序设计语言 - 测试](https://doc.rust-lang.org/book/ch11-00-testing.html)

## ⏭️ 下一步
恭喜你完成了第一阶段（基础知识）的所有内容！你已经掌握了 Rust 的变量、所有权、类型系统、泛型、Trait、生命周期和测试。

接下来，我们将进入第二阶段：**进阶概念**。我们将探索函数式编程特性。

下一节: [Day 16: 闭包](../Day16-30/16.Closures/README.md)
