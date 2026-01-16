# Day 80: 博客系统完整版

## 项目简介
这是一个基于 Rust 的现代化博客 API 系统，采用了 Rust Web 开发的最佳实践。

## 技术栈
- **Web 框架**: Axum 0.7
- **数据库**: SQLx (SQLite)
- **认证**: JWT (Json Web Token) + Argon2 (密码哈希)
- **异步运行时**: Tokio
- **序列化**: Serde
- **日志**: Tracing
- **错误处理**: Anyhow + Thiserror
- **测试**: Tower + Tokio Test

## 功能特性
- 用户注册与登录
- JWT 认证保护
- 文章 CRUD 操作
- 密码加密存储
- 数据库迁移
- 集成测试覆盖

## 运行指南

### 1. 准备环境
确保安装了 Rust 工具链。

### 2. 运行项目
```bash
cd blog-api
cargo run
```
服务将在 `http://localhost:3000` 启动。

### 3. 运行测试
```bash
cargo test
```

## API 接口

### 认证
- `POST /api/auth/register` - 注册
- `POST /api/auth/login` - 登录

### 文章
- `GET /api/posts` - 获取文章列表
- `GET /api/posts/:id` - 获取文章详情
- `POST /api/posts` - 创建文章 (需认证)
- `PUT /api/posts/:id` - 更新文章 (需认证)
- `DELETE /api/posts/:id` - 删除文章 (需认证)

## 目录结构
- `src/api`: API 路由处理函数
- `src/models`: 数据库模型
- `src/repository`: 数据访问层
- `src/utils`: 工具函数 (JWT, Password)
- `src/middleware`: 中间件
- `tests`: 集成测试
- `migrations`: SQL 迁移文件

## 总结
本项目展示了如何构建一个结构清晰、可测试、功能完整的 Rust Web 应用。
