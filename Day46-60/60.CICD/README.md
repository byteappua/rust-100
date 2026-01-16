# Day 60: 项目发布与 CI/CD

[![CI](https://github.com/yourusername/mini-redis-cicd/workflows/CI/badge.svg)](https://github.com/yourusername/mini-redis-cicd/actions)
[![Coverage](https://codecov.io/gh/yourusername/mini-redis-cicd/branch/main/graph/badge.svg)](https://codecov.io/gh/yourusername/mini-redis-cicd)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

恭喜你完成了 Mini-Redis 项目的开发！在本节中，我们将学习如何将项目发布到 crates.io，并设置持续集成/持续部署 (CI/CD) 流水线。

## 1. 发布到 Crates.io

### 1.1 准备工作

在发布之前，确保你的 `Cargo.toml` 包含必要的元数据：

```toml
[package]
name = "mini-redis"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A minimal Redis implementation in Rust for learning purposes"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/mini-redis"
documentation = "https://docs.rs/mini-redis"
readme = "README.md"
keywords = ["redis", "database", "async", "tokio"]
categories = ["database", "network-programming"]

# 排除不需要发布的文件
exclude = [
    "tests/*",
    "benches/*",
    "examples/*",
    ".github/*",
]
```

### 1.2 登录 Crates.io

```bash
# 首先在 https://crates.io 注册账号并获取 API Token
cargo login <your-api-token>
```

### 1.3 检查与发布

```bash
# 检查包是否可以发布
cargo publish --dry-run

# 正式发布
cargo publish
```

## 2. GitHub Actions CI/CD

### 2.1 基础 CI 配置

创建 `.github/workflows/ci.yml`：

```yaml
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  check:
    name: Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo check --all-features

  test:
    name: Test Suite
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --all-features

  fmt:
    name: Rustfmt
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all -- --check

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --all-features -- -D warnings

  docs:
    name: Documentation
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo doc --no-deps --all-features
        env:
          RUSTDOCFLAGS: -D warnings
```

### 2.2 跨平台测试

```yaml
  cross-platform:
    name: Test on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --all-features
```

### 2.3 自动发布

创建 `.github/workflows/release.yml`：

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    name: Publish to crates.io
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo publish
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}

  release:
    name: Create GitHub Release
    runs-on: ubuntu-latest
    needs: publish
    steps:
      - uses: actions/checkout@v4
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          generate_release_notes: true
```

## 3. 代码质量工具

### 3.1 Pre-commit Hooks

安装 `cargo-husky` 来自动运行检查：

```bash
cargo add cargo-husky --dev
```

在 `Cargo.toml` 中配置：

```toml
[dev-dependencies.cargo-husky]
version = "1"
default-features = false
features = ["precommit-hook", "run-cargo-test", "run-cargo-clippy", "run-cargo-fmt"]
```

### 3.2 代码覆盖率

使用 `cargo-tarpaulin` 生成覆盖率报告：

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

添加到 CI：

```yaml
  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
      - name: Generate coverage
        run: cargo tarpaulin --out Xml
      - name: Upload to Codecov
        uses: codecov/codecov-action@v3
```

## 4. 版本管理

### 4.1 语义化版本

遵循 [SemVer](https://semver.org/) 规范：
- **MAJOR**: 不兼容的 API 变更
- **MINOR**: 向后兼容的功能新增
- **PATCH**: 向后兼容的问题修复

### 4.2 Changelog

使用 `git-cliff` 自动生成 CHANGELOG：

```bash
cargo install git-cliff
git cliff --init
git cliff -o CHANGELOG.md
```

### 4.3 发布流程

```bash
# 1. 更新版本号
cargo set-version 0.2.0

# 2. 生成 Changelog
git cliff -o CHANGELOG.md

# 3. 提交并打标签
git add .
git commit -m "chore: release v0.2.0"
git tag v0.2.0

# 4. 推送触发 CI/CD
git push origin main --tags
```

## 5. Docker 支持

创建 `Dockerfile`：

```dockerfile
# Build stage
FROM rust:1.75-alpine AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release

# Runtime stage
FROM alpine:3.19
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/mini-redis /usr/local/bin/
EXPOSE 6379
CMD ["mini-redis"]
```

构建并运行：

```bash
docker build -t mini-redis .
docker run -p 6379:6379 mini-redis
```

## 6. 项目总结

经过 15 天的开发，我们的 Mini-Redis 项目已经具备：

| 功能 | 状态 |
|------|------|
| RESP 协议解析 | ✅ |
| 基础命令 (GET/SET/DEL) | ✅ |
| 过期时间支持 | ✅ |
| 异步网络层 | ✅ |
| 并发控制 | ✅ |
| AOF 持久化 | ✅ |
| Pub/Sub | ✅ |
| Client SDK | ✅ |
| 性能基准测试 | ✅ |
| 集群模式 | ✅ |
| 哨兵高可用 | ✅ |
| TLS 加密 | ✅ |
| 完善文档 | ✅ |
| CI/CD | ✅ |

## 下一步

第四阶段到此结束！在接下来的第五阶段（Day 61-80），我们将进入 **Web 开发实战**，使用 Axum 或 Actix-web 构建完整的 RESTful API 应用。

---

**恭喜完成 Mini-Redis 项目！** 🎉
