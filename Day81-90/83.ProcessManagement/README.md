# Day 83: 进程管理与信号处理

## 学习目标
- 掌握 Rust 中创建和管理子进程的方法
- 理解进程间通信 (IPC) 的基本方式（管道）
- 学习如何捕获和处理 Unix 信号 (Signals)
- 了解守护进程 (Daemon) 的概念与实现

## 核心概念

### 1. 进程创建与管道通信
Rust 标准库 `std::process::Command` 提供了强大的进程创建功能。通过配置 `Stdio`，我们可以重定向子进程的输入输出流，从而实现父子进程间的通信。

- **Spawn**: 异步启动进程。
- **Output**: 启动进程并等待其完成，收集输出。
- **Stdio::piped()**: 创建管道，允许父进程读写子进程的标准输入/输出。

### 2. 信号处理 (Signal Handling)
Unix 信号是操作系统向进程发送的异步通知（如 SIGINT, SIGTERM）。Rust 标准库对信号的支持较为基础，通常推荐使用 `signal-hook` crate 来处理 POSIX 信号。

- **SIGINT**: 中断信号 (Ctrl+C)。
- **SIGTERM**: 终止信号。
- **处理方式**: 通常在一个独立的线程中阻塞等待信号，或者注册回调。

### 3. 守护进程 (Daemon)
守护进程是在后台运行的长期服务进程，不与任何控制终端关联。创建守护进程通常涉及以下步骤：
1. `fork` 并退出父进程。
2. 调用 `setsid` 创建新会话。
3. 更改工作目录。
4. 重设文件权限掩码 (umask)。
5. 重定向标准 IO 流。

Rust 生态中 `daemonize` crate 封装了这些复杂操作。

## 代码示例

本节代码演示了上述三个核心概念的用法。

### 启动子进程并使用管道

```rust
use std::process::{Command, Stdio};
use std::io::{Write, Read};

fn run_pipe() -> std::io::Result<()> {
    // 启动 'tr' 命令，将小写转换为大写
    let mut child = Command::new("tr")
        .arg("a-z")
        .arg("A-Z")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // 写入数据到子进程 stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"hello rust world")?;
    }

    // 读取子进程 stdout
    let output = child.wait_with_output()?;
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
```

### 捕获 SIGINT 信号

```rust
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::thread;

fn handle_signals() -> std::io::Result<()> {
    let mut signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            // 执行清理操作...
            break;
        }
    });

    Ok(())
}
```

## 运行示例

进入 `process-demo` 目录并运行：

```bash
cd process-demo
# 运行 IPC 和信号处理演示
cargo run

# 运行守护进程模式
cargo run -- daemon
```

### 预期输出

1. **IPC 示例**: 程序会调用 `tr` 命令，将 "hello process world" 转换为大写并输出。
2. **信号示例**: 程序会等待 5 秒。在此期间如果你按下 `Ctrl+C`，程序会捕获该信号并打印提示信息，而不是直接强行终止。
3. **守护进程**: 如果运行 `daemon` 模式，程序将转入后台运行。你可以查看 `/tmp/daemon.out` 确认其状态。

## 练习
1. 修改 IPC 示例，让父进程连续发送多行数据，子进程实时处理并返回。
2. 实现一个能够优雅关闭（Graceful Shutdown）的 Web 服务器原型，接收 SIGTERM 信号后停止接受新连接并等待现有请求完成。
3. 尝试使用 Unix Domain Socket (`std::os::unix::net::UnixStream`) 进行更高效的本地进程间通信。

## 下一步
Day 84 将深入讲解原始套接字 (Raw Sockets) 与网络协议实现。
