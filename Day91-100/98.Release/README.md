# Day 98: 发布准备与版本管理

本项目 (**DTask**) 的发布准备阶段。

## 开发目标

1.  **发布制品**: 准备发布所需的二进制文件和资源。
2.  **Docker 镜像**: 构建生产可用的 Docker 镜像。
3.  **版本管理**: 维护 CHANGELOG 和版本号。
4.  **发布清单**: 制定发布前的检查流程。

## 完成内容

### 1. Dockerfile

-   创建了多阶段构建 (Multi-stage Build) 的 `Dockerfile`。
    -   **构建阶段**: 使用 `rust:latest` 编译项目。
    -   **运行阶段**: 使用 `debian:bookworm-slim` 作为轻量级运行时环境。
-   配置了必要的运行时依赖 (如 `libssl`, `ca-certificates`)。

### 2. 发布文档

-   **CHANGELOG.md**: 记录了版本 `1.0.0` 的变更日志，包含 Added, Changed, Fixed 等部分。
-   **RELEASE_CHECKLIST.md**: 列出了发布前必须执行的检查项（测试、文档、版本号等）。

### 3. CI/CD 配置 (模拟)

-   虽然本项目未连接真实的 GitHub Actions Runner，但提供了 `release.yml` 的配置示例，用于自动创建 GitHub Release。

## 运行方式

### 构建 Docker 镜像

```bash
cd Day91-100/98.Release/dtask
docker build -t dtask:1.0.0 .
```

### 运行 Docker 容器

```bash
docker run -p 3000:3000 -p 50051:50051 dtask:1.0.0
```

## 下一步

Day 99 将对整个项目进行复盘和展示。
