# 第六阶段：系统编程与拓展 (Days 81-90) - 完整概览

## 阶段目标
深入系统底层编程，掌握性能优化技术和工程化实践。

## 学习路径

### Week 1: 系统编程 (Days 81-85)

#### Day 81: 操作系统概念回顾
**核心内容：**
- 进程与线程
- 内存管理
- 文件系统
- I/O 模型
- 系统调用

**Rust 系统编程特点：**
```rust
use std::process::Command;
use std::env;

fn main() {
    // 获取环境变量
    let path = env::var("PATH").unwrap();
    println!("PATH: {}", path);
    
    // 执行系统命令
    let output = Command::new("ls")
        .arg("-la")
        .output()
        .expect("Failed to execute command");
    
    println!("Status: {}", output.status);
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
}
```

#### Day 82: 文件系统操作深入
**核心内容：**
- 文件元数据
- 目录遍历
- 符号链接
- 文件权限
- 内存映射文件

**示例代码：**
```rust
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::os::unix::fs::PermissionsExt;

// 递归遍历目录
fn walk_dir(path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            walk_dir(&path)?;
        } else {
            println!("File: {:?}", path);
        }
    }
    Ok(())
}

// 内存映射文件
use memmap2::MmapOptions;

fn mmap_file(path: &Path) -> io::Result<()> {
    let file = File::open(path)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    
    // 直接访问内存
    let data = &mmap[0..100];
    println!("First 100 bytes: {:?}", data);
    
    Ok(())
}

// 文件锁
use fs2::FileExt;

fn file_lock_example() -> io::Result<()> {
    let file = File::open("data.txt")?;
    
    // 独占锁
    file.lock_exclusive()?;
    // 执行操作
    file.unlock()?;
    
    Ok(())
}
```

#### Day 83: 进程管理与信号处理
**核心内容：**
- 进程创建和管理
- 进程间通信（IPC）
- 信号处理
- 守护进程

**示例代码：**
```rust
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

// 管道通信
fn pipe_example() -> io::Result<()> {
    let mut child = Command::new("ls")
        .stdout(Stdio::piped())
        .spawn()?;
    
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            println!("Line: {}", line?);
        }
    }
    
    child.wait()?;
    Ok(())
}

// 信号处理
use signal_hook::{consts::SIGINT, iterator::Signals};

fn signal_handler() -> Result<()> {
    let mut signals = Signals::new(&[SIGINT])?;
    
    for sig in signals.forever() {
        match sig {
            SIGINT => {
                println!("Received SIGINT, shutting down...");
                break;
            }
            _ => unreachable!(),
        }
    }
    
    Ok(())
}

// 守护进程
use daemonize::Daemonize;

fn create_daemon() -> Result<()> {
    let daemonize = Daemonize::new()
        .pid_file("/tmp/myapp.pid")
        .working_directory("/tmp");
    
    match daemonize.start() {
        Ok(_) => println!("Daemon started"),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    Ok(())
}
```

#### Day 84: 原始套接字 (Raw Sockets)
**核心内容：**
- TCP/UDP 底层操作
- 原始套接字编程
- 网络协议实现
- 数据包捕获

**示例代码：**
```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

// 低级 TCP 服务器
fn raw_tcp_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    
    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        
        stream.write_all(&buffer[0..n])?;
    }
    
    Ok(())
}

// 使用 socket2 进行更底层的控制
use socket2::{Socket, Domain, Type, Protocol};

fn create_raw_socket() -> io::Result<Socket> {
    let socket = Socket::new(
        Domain::IPV4,
        Type::STREAM,
        Some(Protocol::TCP),
    )?;
    
    socket.set_nonblocking(true)?;
    socket.set_reuse_address(true)?;
    
    Ok(socket)
}
```

#### Day 85: 嵌入式 Rust 简介
**核心内容：**
- no_std 环境
- 嵌入式 HAL
- 裸机编程
- RTIC 框架

**示例代码：**
```rust
#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // 嵌入式主循环
    loop {
        // 硬件操作
    }
}

// 使用 embedded-hal
use embedded_hal::digital::v2::OutputPin;

fn blink_led<P: OutputPin>(led: &mut P) {
    led.set_high().ok();
    delay_ms(1000);
    led.set_low().ok();
    delay_ms(1000);
}
```

### Week 2: 性能与工程化 (Days 86-90)

#### Day 86: 性能分析工具 (Flamegraph)
**核心内容：**
- CPU 性能分析
- 内存分析
- Flamegraph 生成
- perf 工具使用

**工具使用：**
```bash
# 安装 cargo-flamegraph
cargo install flamegraph

# 生成火焰图
cargo flamegraph --bin myapp

# 使用 perf
perf record -g ./target/release/myapp
perf report

# 使用 valgrind
valgrind --tool=callgrind ./target/release/myapp
```

**代码示例：**
```rust
use std::time::Instant;

fn benchmark_function() {
    let start = Instant::now();
    
    // 执行操作
    expensive_operation();
    
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

// 使用 criterion 进行基准测试
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
```

#### Day 87: 内存优化技巧
**核心内容：**
- 内存布局优化
- 零拷贝技术
- 内存池
- 自定义分配器

**示例代码：**
```rust
// 内存布局优化
#[repr(C)]
struct OptimizedStruct {
    a: u64,  // 8 bytes
    b: u32,  // 4 bytes
    c: u16,  // 2 bytes
    d: u8,   // 1 byte
    // 总共 16 bytes (带对齐)
}

// 零拷贝
use bytes::{Bytes, BytesMut};

fn zero_copy_example() {
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(b"Hello");
    
    // 零拷贝转换
    let frozen: Bytes = buf.freeze();
}

// 对象池
use object_pool::Pool;

struct Connection {
    // 连接数据
}

fn pool_example() {
    let pool = Pool::new(10, || Connection::new());
    
    let conn = pool.pull();
    // 使用连接
    // 自动归还到池中
}

// 自定义分配器
use std::alloc::{GlobalAlloc, Layout};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // 自定义分配逻辑
        std::alloc::System.alloc(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        std::alloc::System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;
```

#### Day 88: SIMD 与底层优化
**核心内容：**
- SIMD 指令集
- 向量化计算
- 编译器优化
- 内联汇编

**示例代码：**
```rust
// 使用 SIMD
use std::arch::x86_64::*;

#[target_feature(enable = "avx2")]
unsafe fn simd_add(a: &[f32], b: &[f32], result: &mut [f32]) {
    for i in (0..a.len()).step_by(8) {
        let va = _mm256_loadu_ps(a.as_ptr().add(i));
        let vb = _mm256_loadu_ps(b.as_ptr().add(i));
        let vr = _mm256_add_ps(va, vb);
        _mm256_storeu_ps(result.as_mut_ptr().add(i), vr);
    }
}

// 使用 packed_simd
use packed_simd::*;

fn vectorized_sum(data: &[f32]) -> f32 {
    let mut sum = f32x8::splat(0.0);
    
    for chunk in data.chunks_exact(8) {
        sum += f32x8::from_slice_unaligned(chunk);
    }
    
    sum.sum()
}

// 内联汇编
use std::arch::asm;

fn inline_asm_example() {
    let x: u64;
    unsafe {
        asm!(
            "mov {}, 42",
            out(reg) x,
        );
    }
    println!("x = {}", x);
}

// 编译器优化提示
#[inline(always)]
fn hot_function() {
    // 总是内联
}

#[cold]
fn error_handler() {
    // 标记为冷路径
}

#[inline(never)]
fn debug_function() {
    // 从不内联
}
```

#### Day 89: 持续集成流水线
**核心内容：**
- GitHub Actions 配置
- 自动化测试
- 代码覆盖率
- 自动发布

**GitHub Actions 配置：**
```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        components: rustfmt, clippy
    
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Check formatting
      run: cargo fmt -- --check
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
    
    - name: Run tests
      run: cargo test --verbose
    
    - name: Run tests with coverage
      run: |
        cargo install cargo-tarpaulin
        cargo tarpaulin --out Xml
    
    - name: Upload coverage
      uses: codecov/codecov-action@v3

  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
    - uses: actions/checkout@v3
    - name: Build
      run: cargo build --release --verbose

  release:
    needs: [test, build]
    runs-on: ubuntu-latest
    if: startsWith(github.ref, 'refs/tags/')
    steps:
    - uses: actions/checkout@v3
    - name: Build release
      run: cargo build --release
    - name: Create Release
      uses: softprops/action-gh-release@v1
      with:
        files: target/release/myapp
```

#### Day 90: Rust 编译优化与配置
**核心内容：**
- Cargo.toml 优化配置
- 编译时间优化
- 二进制大小优化
- 交叉编译

**Cargo.toml 优化：**
```toml
[profile.release]
opt-level = 3              # 最高优化级别
lto = true                 # 链接时优化
codegen-units = 1          # 单个代码生成单元
strip = true               # 移除符号信息
panic = 'abort'            # panic 时直接终止

[profile.release-small]
inherits = "release"
opt-level = 'z'            # 优化大小
lto = true
codegen-units = 1
strip = true

[profile.dev]
opt-level = 0              # 开发时不优化
debug = true               # 包含调试信息

[profile.dev.package."*"]
opt-level = 2              # 依赖包使用优化

# 编译时间优化
[build]
incremental = true         # 增量编译
pipelining = true          # 流水线编译

# 依赖优化
[dependencies]
serde = { version = "1", features = ["derive"], default-features = false }
```

**编译优化技巧：**
```bash
# 使用 sccache 缓存编译结果
cargo install sccache
export RUSTC_WRAPPER=sccache

# 使用 mold 链接器加速链接
cargo install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"

# 并行编译
cargo build -j 8

# 交叉编译
rustup target add x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl --release

# 检查二进制大小
cargo bloat --release
cargo bloat --release --crates

# 分析编译时间
cargo build --timings
```

## 性能优化检查清单

- [ ] 使用 release 模式编译
- [ ] 启用 LTO
- [ ] 优化热路径代码
- [ ] 减少不必要的克隆
- [ ] 使用合适的数据结构
- [ ] 避免不必要的分配
- [ ] 使用 SIMD 加速计算密集型任务
- [ ] 实现零拷贝传输
- [ ] 使用对象池复用对象
- [ ] 性能测试和基准测试

## 工程化最佳实践

### 代码质量
```bash
# 格式化
cargo fmt

# Lint 检查
cargo clippy -- -D warnings

# 安全审计
cargo audit

# 依赖更新
cargo update
cargo outdated
```

### 文档
```bash
# 生成文档
cargo doc --open

# 文档测试
cargo test --doc
```

### 测试
```bash
# 单元测试
cargo test

# 集成测试
cargo test --test integration_test

# 基准测试
cargo bench
```

## 学习资源

### 书籍
- Programming Rust (2nd Edition)
- Rust for Rustaceans
- The Rustonomicon

### 工具
- [cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph)
- [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat)
- [cargo-audit](https://github.com/rustsec/rustsec)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)

### 文档
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/)

## 下一阶段预告

第七阶段（Days 91-100）是毕业设计阶段，将综合运用所有学到的知识完成一个完整的项目。

准备好迎接最后的挑战！🚀
