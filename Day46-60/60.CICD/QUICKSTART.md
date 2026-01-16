# 快速入门指南

## 本地开发

### 1. 克隆项目

```bash
git clone https://github.com/yourusername/mini-redis-cicd.git
cd mini-redis-cicd
```

### 2. 构建项目

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release
```

### 3. 运行项目

```bash
# 开发模式
cargo run

# 发布模式
cargo run --release
```

### 4. 运行测试

```bash
# 运行所有测试
cargo test

# 运行测试并显示输出
cargo test -- --nocapture

# 运行特定测试
cargo test test_valid_semver
```

## 代码质量检查

### 格式化代码

```bash
# 检查格式
cargo fmt --all -- --check

# 自动格式化
cargo fmt --all
```

### Clippy 检查

```bash
# 运行 clippy
cargo clippy --all-features

# 将警告视为错误
cargo clippy --all-features -- -D warnings
```

### 生成文档

```bash
# 生成文档
cargo doc --no-deps --all-features

# 生成并打开文档
cargo doc --no-deps --all-features --open
```

## 使用脚本

### 运行所有检查

```bash
# Linux/macOS
chmod +x scripts/check.sh
./scripts/check.sh

# Windows (PowerShell)
powershell -ExecutionPolicy Bypass -File scripts/check.ps1
```

### 发布新版本

```bash
# Linux/macOS
chmod +x scripts/release.sh
./scripts/release.sh 0.2.0

# Windows (PowerShell)
powershell -ExecutionPolicy Bypass -File scripts/release.ps1 0.2.0
```

## Docker 使用

### 构建镜像

```bash
docker build -t mini-redis-cicd:latest .
```

### 运行容器

```bash
docker run --rm mini-redis-cicd:latest
```

### 使用 Docker Compose

```bash
docker-compose up
```

## CI/CD 工作流

### GitHub Actions

项目包含以下工作流：

1. **CI** (`.github/workflows/ci.yml`)
   - 代码检查
   - 运行测试
   - 格式化检查
   - Clippy 检查
   - 文档生成
   - 跨平台测试

2. **Release** (`.github/workflows/release.yml`)
   - 自动发布到 crates.io
   - 创建 GitHub Release

3. **Coverage** (`.github/workflows/coverage.yml`)
   - 生成代码覆盖率报告
   - 上传到 Codecov

### 触发 CI

```bash
# 推送到主分支触发 CI
git push origin main

# 创建 Pull Request 触发 CI
git checkout -b feature/new-feature
git push origin feature/new-feature
# 然后在 GitHub 上创建 PR
```

### 发布新版本

```bash
# 1. 更新版本号并提交
./scripts/release.sh 0.2.0

# 2. 推送到 GitHub（触发 Release 工作流）
git push origin main --tags
```

## 常用命令别名

在 `.cargo/config.toml` 中定义了以下别名：

```bash
# 检查所有目标
cargo check-all

# 测试所有目标
cargo test-all

# 发布构建
cargo build-release
```

## 故障排除

### 编译错误

```bash
# 清理构建缓存
cargo clean

# 更新依赖
cargo update

# 重新构建
cargo build
```

### 测试失败

```bash
# 显示详细输出
cargo test -- --nocapture

# 运行单个测试
cargo test test_name -- --nocapture
```

### 格式化问题

```bash
# 自动修复格式
cargo fmt --all
```

### Clippy 警告

```bash
# 查看详细警告
cargo clippy --all-features -- -W clippy::all

# 自动修复（实验性）
cargo clippy --fix
```

## 性能分析

### 基准测试

```bash
# 运行基准测试（需要添加 benches/）
cargo bench
```

### 代码覆盖率

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html

# 打开报告
# Linux/macOS: open tarpaulin-report.html
# Windows: start tarpaulin-report.html
```

## 更多资源

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Cargo 文档](https://doc.rust-lang.org/cargo/)
- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [crates.io 发布指南](https://doc.rust-lang.org/cargo/reference/publishing.html)
