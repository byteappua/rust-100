# Day 09 练习题

这些练习将帮助你巩固对 Rust 模块系统的理解。

## 练习 1: 可见性挑战 (Visibility)

文件: `src/exercise1.rs`

这个程序无法编译，因为通过私有模块访问了项目。修复它，使代码可以通过编译。
你需要添加 `pub` 关键字在正确的地方。

## 练习 2: 使用 `use` 简化路径

文件: `src/exercise2.rs`

代码中使用了冗长的绝对路径。请使用 `use` 关键字引入必要的模块或函数，使 `main` 函数中的调用更加简洁。

## 练习 3: 相对路径与 super

文件: `src/exercise3.rs`

在 `back_of_house` 模块中，我们需要调用父模块的 `server_order` 函数。请修复代码中的路径问题。

## ✅ 验证你的答案

你可以使用 rustc 编译每个练习文件来验证你的答案：

```bash
rustc src/exercise1.rs && ./exercise1
rustc src/exercise2.rs && ./exercise2
rustc src/exercise3.rs && ./exercise3
```

或者查看 `../solutions` 目录下的参考答案。
