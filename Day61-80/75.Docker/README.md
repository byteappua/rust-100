# Day 75: 容器化部署 (Docker)

## 学习目标
- 理解容器化技术及其在 Rust 项目中的应用
- 掌握编写高效 Rust Dockerfile 的技巧（多阶段构建）
- 学习使用 Docker Compose 编排服务
- 了解如何在生产环境中部署 Rust 应用

## 核心概念

### 1. Dockerfile 与多阶段构建
Rust 编译产物是二进制文件，不需要运行时环境（如 Node.js 或 Python 解释器）。
**多阶段构建 (Multi-stage builds)** 是 Rust 容器化的最佳实践：
1. **Builder Stage**: 使用包含 Rust 编译器 (`rust:slim`) 的大镜像进行编译。
2. **Runtime Stage**: 将编译好的二进制文件复制到轻量级镜像 (`debian:slim` 或 `alpine`) 中运行。

这样可以显著减小最终镜像的体积（从 >1GB 减小到 <100MB）。

### 2. 依赖缓存
为了加快构建速度，通常先拷贝 `Cargo.toml` 和 `Cargo.lock` 并编译依赖，然后再拷贝源码编译应用。利用 Docker 的层缓存机制，当源码改变但依赖未变时，可以跳过依赖编译步骤。

### 3. Docker Compose
用于定义和运行多容器 Docker 应用程序。通过 `docker-compose.yml` 文件配置应用服务。

## 代码示例

本节包含一个简单的 Axum Web 服务及其 Dockerfile。

### Dockerfile 关键部分

```dockerfile
# Builder
FROM rust:1.80-slim-bookworm as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/docker-demo /usr/local/bin/app
CMD ["app"]
```

## 运行示例

进入 `docker-demo` 目录：

### 1. 本地运行 (Cargo)
```bash
cd docker-demo
cargo run
# 访问 http://localhost:3000
```

### 2. 构建 Docker 镜像
```bash
docker build -t rust-docker-demo .
```

### 3. 运行容器
```bash
docker run -p 3000:3000 rust-docker-demo
# 访问 http://localhost:3000
```

你应当看到返回的 JSON 中包含主机名，在容器中运行时这通常是容器 ID。

## 练习
1. 为该项目添加 `docker-compose.yml`，并集成一个 Redis 服务。
2. 尝试使用 `distroless` 镜像作为基础镜像 (`gcr.io/distroless/cc-debian12`) 以进一步减小体积并提高安全性。
3. 编写 GitHub Actions 脚本，自动构建并推送镜像到 Docker Hub 或 GHCR。

## 下一步
Day 76-80 将开始完整的博客系统项目实战。
