# Day 44: 命令行参数解析 (Clap)

在 Rust 中，编写命令行工具 (CLI) 是一件非常愉快的事情，而 **`clap`** (Command Line Argument Parser) 是这个领域绝对的事实标准。它功能强大、性能优异，并且能自动生成帮助信息。

## 1. `clap` 的两种风格

`clap` 提供了两种主要的使用方式：

1.  **Builder API**: 通过函数链式调用构建参数解析器。灵活性高，编译速度稍快，但代码较冗长。
2.  **Derive API**: 使用 Rust 的宏 (`derive`) 直接在结构体上定义参数。代码简洁直观，类型安全，是目前推荐的方式。

我们今天主要介绍 **Derive API**。

## 2. 核心概念

在使用 Derive API 时，主要会用到三个 trait：

*   `Parser`: 顶级结构体，代表整个命令行程序。
*   `Subcommand`: 用于定义子命令（如 `git commit`, `git push`）。通常是一个枚举。
*   `Args`: 用于定义一组参数。可以被嵌入到 `Parser` 或 `Subcommand` 中。

## 3. 基本用法

首先需要在 `Cargo.toml` 中添加依赖，并开启 `derive` 特性：

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

### 示例代码

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 可选的名字
    #[arg(short, long)]
    name: Option<String>,

    /// 计数次数 (默认 1)
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 执行测试
    Test {
        /// 是否列出详情
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {}", name);
    }

    // ...
}
```

## 4. 运行效果

`clap` 会自动生成 `--help` 信息：

```bash
$ cargo run -- --help
Usage: demo [OPTIONS] [COMMAND]

Commands:
  test  执行测试
  help  Print this message or the help of the given subcommand(s)

Options:
  -n, --name <NAME>    可选的名字
  -c, --count <COUNT>  计数次数 (默认 1) [default: 1]
  -h, --help           Print help
  -V, --version        Print version
```
