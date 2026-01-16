# Day 82: 文件系统操作深入

## 学习目标
- 掌握 Rust 中的目录遍历技术
- 理解并实现内存映射文件 (Memory Mapped Files)
- 学习文件锁机制以处理并发访问
- 熟悉文件元数据和权限操作

## 核心概念

### 1. 目录遍历
标准库 `std::fs` 提供了基本的目录操作。递归遍历通常需要手动实现，或者使用 `walkdir` 等第三方库。本示例展示了如何使用标准库实现简单的递归遍历。

### 2. 内存映射文件 (mmap)
内存映射文件允许将文件的一部分或全部直接映射到进程的内存空间。这对于处理大文件或实现高性能 I/O 非常有用。Rust 中常用 `memmap2` crate 来实现。
- **优点**：减少系统调用，像访问内存一样访问文件。
- **Unsafe**：创建 mmap 通常是 `unsafe` 的，因为底层文件可能在映射期间被其他进程修改，导致未定义行为。

### 3. 文件锁
当多个进程尝试同时读写同一个文件时，可能会导致数据损坏。文件锁（File Locking）是一种同步机制。
- **独占锁 (Exclusive Lock)**：通常用于写操作，一次只有一个进程能持有。
- **共享锁 (Shared Lock)**：通常用于读操作，多个进程可以同时持有。
Rust 中可以使用 `fs2` crate 来进行跨平台的文件锁定。

## 代码示例

本节代码演示了上述三个核心概念的用法。

### 递归遍历目录

```rust
use std::fs;
use std::path::Path;
use std::io;

fn walk_dir(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                walk_dir(&path)?;
            } else {
                println!("File: {:?}", path);
            }
        }
    }
    Ok(())
}
```

### 内存映射文件

需要依赖 `memmap2` crate。

```rust
use std::fs::File;
use memmap2::MmapOptions;

fn mmap_file(path: &Path) -> io::Result<()> {
    let file = File::open(path)?;
    // unsafe: 调用者必须确保文件不会被外部修改从而导致数据竞争
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    // 像访问切片一样访问文件内容
    if !mmap.is_empty() {
        println!("First byte: {}", mmap[0]);
    }

    Ok(())
}
```

### 文件锁

需要依赖 `fs2` crate。

```rust
use std::fs::File;
use fs2::FileExt;

fn file_lock_example(path: &str) -> io::Result<()> {
    let file = File::open(path)?;

    // 获取独占锁，如果已被占用则阻塞直到可用
    file.lock_exclusive()?;
    println!("Lock acquired");

    // 执行关键区操作...

    // 解锁
    file.unlock()?;
    println!("Lock released");

    Ok(())
}
```

## 运行示例

进入 `fs-demo` 目录并运行：

```bash
cd fs-demo
cargo run
```

你将看到程序遍历源代码目录，创建测试文件进行内存映射读取，并演示文件锁的获取与释放。

## 练习
1. 修改 `walk_dir` 函数，计算目录中所有文件的总大小。
2. 尝试使用 `notify` crate 监听文件系统的变化（如文件创建、修改）。
3. 编写一个程序，使用 mmap 实现大文件的高效复制。

## 下一步
Day 83 将深入讲解进程管理与信号处理。
