# CLI Application Template

一个功能完整的 Rust CLI 应用模板。

## 特性

- ✅ 命令行参数解析 (clap)
- ✅ 配置文件支持 (TOML)
- ✅ 错误处理 (anyhow + thiserror)
- ✅ 日志记录 (env_logger)
- ✅ 彩色输出 (colored)
- ✅ 进度条 (indicatif)
- ✅ 单元测试和集成测试

## 快速开始

### 安装

```bash
cargo build --release
```

### 使用

```bash
# 查看帮助
./target/release/cli-app --help

# 运行命令
./target/release/cli-app run --input input.txt --output output.txt

# 初始化
./target/release/cli-app init --path ./my-project

# 使用配置文件
./target/release/cli-app --config custom-config.toml run
```

## 配置

创建 `config.toml` 文件:

```toml
app_name = "My CLI App"
version = "0.1.0"

[settings]
verbose = true
max_concurrency = 8
timeout = 60
```

## 开发

### 运行测试

```bash
cargo test
```

### 运行示例

```bash
cargo run -- run --input test.txt
```

### 启用日志

```bash
RUST_LOG=debug cargo run -- run
```

## 项目结构

```
cli-app/
├── src/
│   ├── main.rs      # 入口文件
│   ├── cli.rs       # CLI 定义
│   ├── config.rs    # 配置管理
│   └── error.rs     # 错误类型
├── tests/           # 集成测试
├── Cargo.toml       # 项目配置
└── README.md        # 文档
```

## 自定义

1. 修改 `Cargo.toml` 中的项目信息
2. 在 `cli.rs` 中定义你的命令
3. 在 `main.rs` 中实现命令逻辑
4. 在 `config.rs` 中添加配置项

## 许可证

MIT OR Apache-2.0
