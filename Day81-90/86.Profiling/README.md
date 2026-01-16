# Day 86: 性能分析工具 (Flamegraph)

## 学习目标
- 掌握性能分析工具
- 学习火焰图分析
- 理解性能瓶颈识别
- 掌握基准测试方法

## 性能分析工具概览

### 工具列表

| 工具 | 用途 | 平台 |
|------|------|------|
| **cargo-flamegraph** | CPU 火焰图 | Linux, macOS |
| **perf** | Linux 性能分析 | Linux |
| **Instruments** | macOS 性能分析 | macOS |
| **valgrind** | 内存分析 | Linux, macOS |
| **heaptrack** | 堆内存分析 | Linux |
| **criterion** | 基准测试 | All |

## Flamegraph 安装和使用

### 安装

```bash
# 安装 cargo-flamegraph
cargo install flamegraph

# Linux 需要安装 perf
sudo apt-get install linux-tools-common linux-tools-generic

# macOS 使用 DTrace (已内置)
```

### 基本使用

```bash
# 生成火焰图
cargo flamegraph --bin myapp

# 指定输出文件
cargo flamegraph --bin myapp -o my-flamegraph.svg

# 分析特定功能
cargo flamegraph --bin myapp -- --bench

# 使用 release 模式
cargo flamegraph --release --bin myapp
```

### 示例程序

```rust
// src/main.rs
use std::time::Duration;
use std::thread;

fn expensive_computation() {
    let mut sum = 0u64;
    for i in 0..10_000_000 {
        sum = sum.wrapping_add(i);
    }
}

fn io_operation() {
    thread::sleep(Duration::from_millis(100));
}

fn process_data() {
    for _ in 0..10 {
        expensive_computation();
    }
}

fn main() {
    println!("Starting performance test...");
    
    for _ in 0..5 {
        process_data();
        io_operation();
    }
    
    println!("Performance test completed");
}
```

## 火焰图解读

### 火焰图结构

```
┌─────────────────────────────────────┐
│         main (100%)                 │  ← 最顶层：程序入口
├─────────────┬───────────────────────┤
│ process_data│    io_operation       │  ← 第二层：主要函数
│   (70%)     │       (30%)           │
├─────────────┤                       │
│ expensive_  │                       │  ← 第三层：具体实现
│ computation │                       │
│   (70%)     │                       │
└─────────────┴───────────────────────┘
```

### 关键指标

- **宽度**: 函数占用 CPU 时间比例
- **颜色**: 通常随机，用于区分不同函数
- **高度**: 调用栈深度

### 分析要点

```rust
// 热点函数识别
// 1. 宽度最大的函数 = CPU 时间最多
// 2. 平顶 = 该函数本身耗时
// 3. 尖顶 = 调用其他函数耗时

// 优化目标
// - 减少宽函数的执行时间
// - 减少函数调用次数
// - 优化算法复杂度
```

## perf 工具使用

### 基本命令

```bash
# 记录性能数据
perf record -g ./target/release/myapp

# 查看报告
perf report

# 实时监控
perf top

# 统计信息
perf stat ./target/release/myapp
```

### 高级用法

```bash
# 记录特定事件
perf record -e cycles,instructions -g ./target/release/myapp

# 记录特定 CPU
perf record -C 0 -g ./target/release/myapp

# 记录特定进程
perf record -p <PID> -g

# 生成火焰图
perf script | stackcollapse-perf.pl | flamegraph.pl > perf.svg
```

## Criterion 基准测试

### 安装和配置

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

### 基本示例

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
    
    c.bench_function("fib_iter 20", |b| {
        b.iter(|| fibonacci_iterative(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 运行基准测试

```bash
# 运行所有基准测试
cargo bench

# 运行特定基准测试
cargo bench --bench my_benchmark

# 保存基线
cargo bench -- --save-baseline my-baseline

# 与基线比较
cargo bench -- --baseline my-baseline
```

### 高级基准测试

```rust
use criterion::{BenchmarkId, Criterion, Throughput};

fn bench_with_input(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");
    
    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                let mut data: Vec<i32> = (0..size).collect();
                b.iter(|| {
                    data.sort();
                });
            },
        );
    }
    
    group.finish();
}
```

## 内存分析

### Valgrind

```bash
# 内存泄漏检测
valgrind --leak-check=full ./target/release/myapp

# 缓存分析
valgrind --tool=cachegrind ./target/release/myapp

# 调用图分析
valgrind --tool=callgrind ./target/release/myapp
```

### Heaptrack

```bash
# 安装
sudo apt-get install heaptrack

# 运行分析
heaptrack ./target/release/myapp

# 查看结果
heaptrack_gui heaptrack.myapp.*.gz
```

## 实战示例

### 1. 字符串处理优化

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// 未优化版本
fn concat_strings_v1(strings: &[&str]) -> String {
    let mut result = String::new();
    for s in strings {
        result = result + s;  // 每次都重新分配
    }
    result
}

// 优化版本 1: 使用 push_str
fn concat_strings_v2(strings: &[&str]) -> String {
    let mut result = String::new();
    for s in strings {
        result.push_str(s);
    }
    result
}

// 优化版本 2: 预分配容量
fn concat_strings_v3(strings: &[&str]) -> String {
    let total_len: usize = strings.iter().map(|s| s.len()).sum();
    let mut result = String::with_capacity(total_len);
    for s in strings {
        result.push_str(s);
    }
    result
}

// 优化版本 3: 使用 join
fn concat_strings_v4(strings: &[&str]) -> String {
    strings.join("")
}

fn benchmark_concat(c: &mut Criterion) {
    let strings: Vec<&str> = vec!["hello"; 1000];
    
    c.bench_function("concat_v1", |b| {
        b.iter(|| concat_strings_v1(black_box(&strings)))
    });
    
    c.bench_function("concat_v2", |b| {
        b.iter(|| concat_strings_v2(black_box(&strings)))
    });
    
    c.bench_function("concat_v3", |b| {
        b.iter(|| concat_strings_v3(black_box(&strings)))
    });
    
    c.bench_function("concat_v4", |b| {
        b.iter(|| concat_strings_v4(black_box(&strings)))
    });
}

criterion_group!(benches, benchmark_concat);
criterion_main!(benches);
```

### 2. 集合操作优化

```rust
use std::collections::{HashMap, HashSet};

// 查找重复元素 - 未优化
fn find_duplicates_v1(numbers: &[i32]) -> Vec<i32> {
    let mut duplicates = Vec::new();
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] == numbers[j] && !duplicates.contains(&numbers[i]) {
                duplicates.push(numbers[i]);
            }
        }
    }
    duplicates
}

// 查找重复元素 - 优化版本
fn find_duplicates_v2(numbers: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();
    
    for &num in numbers {
        if !seen.insert(num) {
            duplicates.insert(num);
        }
    }
    
    duplicates.into_iter().collect()
}

fn benchmark_duplicates(c: &mut Criterion) {
    let numbers: Vec<i32> = (0..1000).cycle().take(10000).collect();
    
    c.bench_function("duplicates_v1", |b| {
        b.iter(|| find_duplicates_v1(black_box(&numbers)))
    });
    
    c.bench_function("duplicates_v2", |b| {
        b.iter(|| find_duplicates_v2(black_box(&numbers)))
    });
}
```

### 3. 并行处理优化

```rust
use rayon::prelude::*;

// 串行处理
fn process_serial(data: &[i32]) -> Vec<i32> {
    data.iter()
        .map(|&x| x * x)
        .filter(|&x| x % 2 == 0)
        .collect()
}

// 并行处理
fn process_parallel(data: &[i32]) -> Vec<i32> {
    data.par_iter()
        .map(|&x| x * x)
        .filter(|&x| x % 2 == 0)
        .collect()
}

fn benchmark_parallel(c: &mut Criterion) {
    let data: Vec<i32> = (0..1_000_000).collect();
    
    c.bench_function("serial", |b| {
        b.iter(|| process_serial(black_box(&data)))
    });
    
    c.bench_function("parallel", |b| {
        b.iter(|| process_parallel(black_box(&data)))
    });
}
```

## 性能分析流程

### 1. 识别瓶颈

```bash
# 生成火焰图
cargo flamegraph --release --bin myapp

# 查看 CPU 使用情况
perf stat cargo run --release
```

### 2. 基准测试

```bash
# 建立基线
cargo bench -- --save-baseline before

# 优化代码后测试
cargo bench -- --baseline before
```

### 3. 验证改进

```rust
// 使用 criterion 比较
use criterion::Criterion;

fn compare_implementations(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison");
    
    group.bench_function("old", |b| b.iter(|| old_implementation()));
    group.bench_function("new", |b| b.iter(|| new_implementation()));
    
    group.finish();
}
```

## 性能优化检查清单

- [ ] 使用 release 模式编译
- [ ] 启用 LTO (Link Time Optimization)
- [ ] 分析火焰图找出热点
- [ ] 减少不必要的分配
- [ ] 使用合适的数据结构
- [ ] 考虑并行处理
- [ ] 缓存重复计算
- [ ] 避免不必要的克隆
- [ ] 使用迭代器而非循环
- [ ] 预分配容量

## 练习

1. 分析一个实际项目的性能瓶颈
2. 优化字符串处理函数
3. 比较不同排序算法的性能
4. 实现并行版本的数据处理
5. 创建完整的基准测试套件

## 下一步

Day 87 将学习内存优化技巧，包括零拷贝、内存池等。
