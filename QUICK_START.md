# Rust 100 天学习计划 - 快速开始

## 🚀 快速开始

### 环境准备

#### 1. 安装 Rust

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# 下载并运行 https://rustup.rs/
```

#### 2. 验证安装

```bash
rustc --version
cargo --version
```

#### 3. 配置编辑器

**VS Code 推荐插件：**
- rust-analyzer
- CodeLLDB
- crates
- Even Better TOML

**配置文件 (.vscode/settings.json):**
```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "editor.formatOnSave": true
}
```

### 学习路径

#### 第一周：基础入门 (Days 1-7)

```bash
# Day 1: Hello World
cargo new hello_world
cd hello_world
cargo run

# Day 2: 变量和类型
cd Day01-15/02.VariablesAndTypes/variables
cargo run

# Day 3: 函数和控制流
cd Day01-15/03.FunctionsAndControlFlow/branches_and_loops
cargo run
```

**每日学习计划：**
1. 阅读当天的 README.md (30分钟)
2. 运行示例代码 (30分钟)
3. 完成练习题 (60分钟)
4. 总结笔记 (15分钟)

#### 第二周：核心概念 (Days 8-15)

**重点内容：**
- Day 4-6: 所有权系统 ⭐⭐⭐
- Day 7-8: 结构体和枚举
- Day 11: 错误处理 ⭐⭐
- Day 13: Traits ⭐⭐

```bash
# 所有权练习
cd Day01-15/04.Ownership/ownership_demo
cargo run

# 错误处理
cd Day01-15/11.ErrorHandling/error_handling_demo
cargo run
```

#### 第三-四周：进阶特性 (Days 16-30)

**重点内容：**
- Day 16-17: 闭包和迭代器 ⭐⭐
- Day 22-25: 并发编程 ⭐⭐⭐
- Day 28-30: 高级特性

```bash
# 并发编程
cd Day16-30/22.Threads
cargo run

# 消息传递
cd Day16-30/23.MessagePassing
cargo run
```

#### 第五-六周：实用技能 (Days 31-45)

**重点内容：**
- Day 35: Async/Await ⭐⭐⭐
- Day 39: Tokio 深入 ⭐⭐
- Day 41: 数据库操作 ⭐⭐

```bash
# 异步编程
cd Day31-45/35.AsyncAwait
cargo run

# 数据库
cd Day31-45/41.Database_SQLx
cargo run
```

#### 第七-八周：Redis 项目 (Days 46-60)

**项目里程碑：**
- Day 46-50: 基础功能
- Day 51-55: 高级功能
- Day 56-60: 集群和部署

```bash
# 运行 Redis 克隆
cd Day46-60/60.CICD
cargo run --release

# 运行测试
cargo test

# 运行基准测试
cargo bench
```

#### 第九-十周：Web 开发 (Days 61-80)

**项目里程碑：**
- Day 61-65: Web 框架基础
- Day 66-70: 数据库和认证
- Day 76-80: 博客系统

```bash
# Web 服务器
cd Day61-80/62.AxumActix
cargo run

# JWT 认证
cd Day61-80/69.JWT
cargo run
```

## 📚 学习资源

### 官方文档
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

### 视频教程
- [Let's Get Rusty](https://www.youtube.com/c/LetsGetRusty)
- [Jon Gjengset](https://www.youtube.com/c/JonGjengset)

### 书籍推荐
- Programming Rust (O'Reilly)
- Rust for Rustaceans
- Zero To Production In Rust

## 🛠️ 常用命令

### Cargo 命令

```bash
# 创建新项目
cargo new project_name
cargo new --lib library_name

# 构建和运行
cargo build              # 调试模式
cargo build --release    # 发布模式
cargo run                # 运行
cargo run --release      # 发布模式运行

# 测试
cargo test               # 运行所有测试
cargo test test_name     # 运行特定测试
cargo test -- --nocapture  # 显示打印输出

# 文档
cargo doc --open         # 生成并打开文档

# 代码检查
cargo check              # 快速检查
cargo clippy             # Lint 检查
cargo fmt                # 格式化代码

# 依赖管理
cargo update             # 更新依赖
cargo tree               # 查看依赖树
```

### Rustup 命令

```bash
# 更新 Rust
rustup update

# 安装工具链
rustup toolchain install stable
rustup toolchain install nightly

# 切换工具链
rustup default stable
rustup default nightly

# 添加目标平台
rustup target add wasm32-unknown-unknown

# 安装组件
rustup component add rustfmt
rustup component add clippy
```

## 🎯 学习技巧

### 1. 理解所有权系统

```rust
// ❌ 错误：值被移动
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);  // 编译错误

// ✅ 正确：使用引用
let s1 = String::from("hello");
let s2 = &s1;
println!("{}", s1);  // OK
```

### 2. 善用编译器提示

```rust
// 编译器会给出详细的错误信息和建议
// 仔细阅读错误信息，它们通常很有帮助
```

### 3. 使用 Clippy

```bash
# Clippy 会给出代码改进建议
cargo clippy

# 示例输出：
# warning: you seem to be trying to use `match` for destructuring a single pattern
# help: try this: `if let Some(x) = option { ... }`
```

### 4. 阅读标准库源码

```rust
// 在 VS Code 中，按住 Ctrl 点击类型名
// 可以跳转到定义查看源码
let v = Vec::new();  // Ctrl+点击 Vec
```

### 5. 实践项目

- 每周完成一个小项目
- 参与开源项目
- 解决 LeetCode Rust 题目

## 🐛 常见问题

### 1. 借用检查器错误

```rust
// 问题：同时存在可变和不可变引用
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);  // 错误！
println!("{}", first);

// 解决：缩短引用的生命周期
let mut v = vec![1, 2, 3];
{
    let first = &v[0];
    println!("{}", first);
}
v.push(4);  // OK
```

### 2. 生命周期注解

```rust
// 问题：编译器无法推断生命周期
fn longest(x: &str, y: &str) -> &str {  // 错误
    if x.len() > y.len() { x } else { y }
}

// 解决：添加生命周期注解
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 3. 异步运行时

```rust
// 问题：没有运行时
async fn hello() {
    println!("Hello");
}

fn main() {
    hello();  // 不会执行
}

// 解决：使用 Tokio
#[tokio::main]
async fn main() {
    hello().await;  // OK
}
```

## 📊 进度追踪

### 每日检查清单

- [ ] 阅读当天教程
- [ ] 运行示例代码
- [ ] 完成练习题
- [ ] 记录学习笔记
- [ ] 提交代码到 Git

### 每周检查清单

- [ ] 复习本周内容
- [ ] 完成周项目
- [ ] 参与社区讨论
- [ ] 阅读相关文章
- [ ] 更新学习博客

### 阶段检查清单

- [ ] 第一阶段 (Days 1-15): 基础掌握
- [ ] 第二阶段 (Days 16-30): 进阶特性
- [ ] 第三阶段 (Days 31-45): 实用技能
- [ ] 第四阶段 (Days 46-60): Redis 项目
- [ ] 第五阶段 (Days 61-80): Web 开发
- [ ] 第六阶段 (Days 81-90): 系统编程
- [ ] 第七阶段 (Days 91-100): 毕业设计

## 🎓 认证和成就

### 完成标准

每个阶段完成后，确保：
1. 理解核心概念
2. 能够独立编写代码
3. 完成所有练习
4. 通过阶段测试

### 项目展示

- 在 GitHub 上创建项目仓库
- 编写完整的 README
- 添加测试和文档
- 分享到社区

## 💬 获取帮助

### 社区资源

- [Rust 官方论坛](https://users.rust-lang.org/)
- [Reddit r/rust](https://www.reddit.com/r/rust/)
- [Rust 中文社区](https://rustcc.cn/)
- [Discord](https://discord.gg/rust-lang)

### 提问技巧

1. 提供完整的错误信息
2. 分享最小可复现示例
3. 说明已尝试的解决方案
4. 使用 [Rust Playground](https://play.rust-lang.org/)

## 🚀 开始学习

```bash
# 克隆项目
git clone https://github.com/yourusername/rust-100-days.git
cd rust-100-days

# 开始 Day 1
cd Day01-15/01.Introduction/hello_world
cargo run

# 祝你学习愉快！🦀
```

---

**记住：学习 Rust 需要耐心，但回报是值得的！**

**Happy Coding! 🦀**
