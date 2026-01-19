# Day 15 练习题

这些练习题包含了内置的测试模块。
你需要运行 `rustc --test src/exerciseX.rs && ./exerciseX` 来运行测试，或者直接使用 `cargo test` 如果你是在 Cargo 项目中。
由于我们是单文件练习，推荐使用 `rustc --test`。

## 练习 1: 基础断言

文件: `src/exercise1.rs`

我们实现了一个简单的计算器 `Calculator`。
请在 `tests` 模块中补充测试用例，验证 `add` 和 `subtract` 方法的正确性。

## 练习 2: 逻辑测试

文件: `src/exercise2.rs`

有一个 `Rectangle` 结构体和 `can_hold` 方法。
编写测试用例 `larger_can_hold_smaller` 和 `smaller_cannot_hold_larger` 来验证逻辑。

## 练习 3: Panic 测试

文件: `src/exercise3.rs`

函数 `verify_percentage` 应该在输入值小于 0.0 或大于 1.0 时 panic。
编写一个测试用例，使用 `#[should_panic]` 属性来验证它确实会 panic。

## ✅ 验证你的答案

注意命令不同，因为我们要运行测试 harness：

```bash
rustc --test src/exercise1.rs && ./exercise1
rustc --test src/exercise2.rs && ./exercise2
rustc --test src/exercise3.rs && ./exercise3
```
