# 快速参考卡片

## 常用命令速查

### 开发命令

```bash
# 构建
cargo build              # Debug 构建
cargo build --release    # Release 构建

# 测试
cargo test              # 运行所有测试
cargo test --lib        # 只运行单元测试
cargo test --test '*'   # 只运行集成测试
cargo test --doc        # 只运行文档测试

# 检查
cargo check             # 快速检查
cargo clippy            # Lint 检查
cargo fmt               # 格式化代码
cargo fmt -- --check    # 检查格式

# 文档
cargo doc               # 生成文档
cargo doc --open        # 生成并打开文档

# 运行
cargo run               # 运行主程序
cargo run --example version_check  # 运行示例

# 性能测试
cargo bench             # 运行基准测试

# 清理
cargo clean             # 清理构建产物
```

### Make 命令

```bash
make help       # 显示帮助
make build      # 构建项目
make test       # 运行测试
make check      # 运行所有检查
make fmt        # 格式化代码
make clippy     # Clippy 检查
make doc        # 生成文档
make run        # 运行项目
make clean      # 清理
make bench      # 性能测试
make coverage   # 代码覆盖率
make ci         # CI 检查
```

### 脚本命令

```bash
# Linux/macOS
./scripts/check.sh          # 基础检查
./scripts/full-check.sh     # 完整检查
./scripts/verify.sh         # 验证
./scripts/release.sh 0.2.0  # 发布

# Windows (PowerShell)
.\scripts\check.ps1
.\scripts\full-check.ps1
.\scripts\verify.ps1
.\scripts\release.ps1 0.2.0
```

### Docker 命令

```bash
# 构建镜像
docker build -t mini-redis-cicd:latest .

# 运行容器
docker run -d -p 6379:6379 mini-redis-cicd:latest

# 查看日志
docker logs mini-redis

# 进入容器
docker exec -it mini-redis sh

# 使用 Compose
docker-compose up -d
docker-compose down
docker-compose logs -f
```

### Git 工作流

```bash
# 创建功能分支
git checkout -b feature/new-feature

# 提交更改
git add .
git commit -m "feat: add new feature"

# 推送分支
git push origin feature/new-feature

# 创建 PR（在 GitHub 上）

# 发布版本
git tag v0.2.0
git push origin v0.2.0
```

## 文件位置速查

### 源代码
- `src/lib.rs` - 库入口
- `src/main.rs` - 主程序

### 测试
- `tests/integration_test.rs` - 集成测试
- `benches/version_benchmark.rs` - 性能测试

### 示例
- `examples/version_check.rs` - 版本检查示例
- `examples/cli_demo.rs` - CLI 演示

### 配置
- `Cargo.toml` - 项目配置
- `rustfmt.toml` - 格式化配置
- `cliff.toml` - Changelog 配置
- `.cargo/config.toml` - Cargo 配置

### CI/CD
- `.github/workflows/ci.yml` - 持续集成
- `.github/workflows/release.yml` - 自动发布
- `.github/workflows/coverage.yml` - 代码覆盖率
- `.github/workflows/docker.yml` - Docker 构建

### 文档
- `README.md` - 项目说明
- `QUICKSTART.md` - 快速入门
- `CONTRIBUTING.md` - 贡献指南
- `DEPLOYMENT.md` - 部署指南
- `BADGES.md` - 徽章指南

## 常见问题速查

### Q: 如何运行完整检查？
```bash
# Linux/macOS
./scripts/full-check.sh

# Windows
.\scripts\full-check.ps1

# 或使用 Make
make ci
```

### Q: 如何修复格式问题？
```bash
cargo fmt --all
```

### Q: 如何修复 Clippy 警告？
```bash
# 查看详细信息
cargo clippy --all-features

# 自动修复（部分）
cargo clippy --fix
```

### Q: 如何生成代码覆盖率？
```bash
# 安装工具
cargo install cargo-tarpaulin

# 生成报告
cargo tarpaulin --out Html
```

### Q: 如何发布新版本？
```bash
# 1. 更新版本号
cargo set-version 0.2.0

# 2. 生成 Changelog
git cliff -o CHANGELOG.md

# 3. 提交并打标签
git add .
git commit -m "chore: release v0.2.0"
git tag v0.2.0

# 4. 推送
git push origin main --tags
```

### Q: 如何添加新的依赖？
```bash
# 添加依赖
cargo add <crate-name>

# 添加开发依赖
cargo add --dev <crate-name>

# 添加构建依赖
cargo add --build <crate-name>
```

### Q: 如何更新依赖？
```bash
# 更新所有依赖
cargo update

# 更新特定依赖
cargo update <crate-name>

# 检查过时依赖
cargo outdated
```

### Q: 如何调试程序？
```bash
# 使用 VS Code
# 1. 打开 src/main.rs
# 2. 设置断点
# 3. 按 F5 开始调试

# 使用命令行
RUST_BACKTRACE=1 cargo run
```

## 环境变量

```bash
# 日志级别
export RUST_LOG=debug
export RUST_LOG=mini_redis_cicd=info

# 回溯信息
export RUST_BACKTRACE=1
export RUST_BACKTRACE=full

# Cargo 配置
export CARGO_TERM_COLOR=always
export CARGO_INCREMENTAL=1
```

## 性能优化

### Cargo.toml 优化
```toml
[profile.release]
opt-level = 3           # 最高优化级别
lto = true              # 链接时优化
codegen-units = 1       # 单个代码生成单元
strip = true            # 移除符号信息
panic = 'abort'         # Panic 时直接终止
```

### 编译加速
```bash
# 使用 sccache
export RUSTC_WRAPPER=sccache

# 使用 mold 链接器（Linux）
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"
```

## 故障排查

### 编译错误
```bash
# 清理并重新构建
cargo clean
cargo build

# 更新工具链
rustup update
```

### 测试失败
```bash
# 显示详细输出
cargo test -- --nocapture

# 运行特定测试
cargo test test_name

# 显示被忽略的测试
cargo test -- --ignored
```

### 性能问题
```bash
# 使用 Release 模式
cargo build --release

# 性能分析
cargo flamegraph
```

## 有用的链接

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Cargo 文档](https://doc.rust-lang.org/cargo/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- [crates.io](https://crates.io/)
- [docs.rs](https://docs.rs/)

## 快捷键（VS Code）

- `F5` - 开始调试
- `Ctrl+Shift+B` - 构建
- `Ctrl+Shift+P` - 命令面板
- `Ctrl+.` - 快速修复
- `F12` - 跳转到定义
- `Shift+F12` - 查找引用

---

**提示**: 将此文件加入书签，方便快速查阅！
