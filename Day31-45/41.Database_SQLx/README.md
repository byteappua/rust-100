# Day 41: 数据库交互 (SQLx)

在 Rust 中，**SQLx** 是一个现代的异步 SQL 数据库 crate。与其他 ORM (如 Diesel) 不同，SQLx 倾向于使用原生 SQL，但提供了强大的类型安全检查。

## 为什么选择 SQLx？

*   **异步设计**: 从头开始构建的 async/await 支持（基于 Tokio 或 async-std）。
*   **编译时检查**: 如果使用 `sqlx::query!` 宏，它可以在编译时连接数据库检查 SQL 语法和类型正确性。
*   **数据库支持**: Postgres, MySQL, SQLite, MSSQL。
*   **非 ORM**: 它不尝试隐藏 SQL，而是拥抱 SQL。

## 核心概念

### 1. 连接池 (`Pool`)

在 Web 应用中，我们通常使用连接池来管理数据库连接，以提高性能。

```rust
let pool = SqlitePoolOptions::new()
    .connect("sqlite::memory:")
    .await?;
```

### 2. 查询 (`query` vs `query_as`)

*   `query`: 返回未类型化的行 (`Row`)。
*   `query_as`: 将结果映射到 Rust 结构体（需要实现 `FromRow` Trait）。

```rust
#[derive(FromRow)]
struct User { id: i32, name: String }

let users: Vec<User> = sqlx::query_as("SELECT * FROM users")
    .fetch_all(&pool)
    .await?;
```

### 3. 参数绑定 (`bind`)

**永远不要拼接 SQL 字符串！** 使用 `bind` 或占位符（SQLite 中是 `?`，Postgres 中是 `$1`）来防止 SQL 注入。

```rust
sqlx::query("INSERT INTO users (name) VALUES (?)")
    .bind("Alice")
    .execute(&pool)
    .await?;
```

## 本日示例

为了简化运行，本示例使用内存中的 SQLite 数据库 (`sqlite::memory:`)，无需安装额外数据库服务。

运行：
```bash
cargo run
```
