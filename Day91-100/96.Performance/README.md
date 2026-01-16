# Day 96: 性能测试与调优

本目录包含经过性能优化的 `dtask` 项目版本。

## 变更内容

1.  **数据库迁移**: 从内存中的 `Vec<Task>` 迁移到 `SQLite`，使用 `sqlx` 进行异步访问。
2.  **Schema 优化**: 添加了 `tasks` 表，并在 `status` 字段上创建了索引以优化查询。
3.  **连接池**: 配置了 `SqlitePool`，并调优了最大连接数。
4.  **构建配置**: 在 `Cargo.toml` 中添加了 `release` 配置，启用了 `LTO` (链接时优化) 和 `strip`。

## 运行方式

1.  **启动服务器**:
    ```bash
    cd Day91-100/96.Performance/dtask
    cargo run --release
    ```

2.  **运行负载测试**:
    ```bash
    # 在另一个终端
    cd Day91-100/96.Performance/dtask
    cargo run --example load_test --release
    ```

## 优化清单

-   [x] **数据库查询优化**: 在 `status` 字段添加索引 (见 migration `20240101000001_add_index.sql`)。
-   [x] **连接池调优**: 设置 `max_connections(50)`。
-   [x] **编译优化**: 启用 `lto = true`, `strip = true`, `opt-level = 3`。
-   [x] **异步 I/O**: 全面使用 `tokio` 和 `sqlx` 的异步能力。

## 基准测试结果

(运行 load_test 示例可查看当前环境下的 QPS)
