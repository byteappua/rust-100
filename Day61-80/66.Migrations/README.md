# Day 66: 数据库迁移 (Migrations)

## 学习目标
- 理解数据库迁移概念
- 掌握 SQLx 迁移系统
- 学习版本控制策略
- 实现回滚机制

## 数据库迁移概念

数据库迁移是管理数据库 schema 变更的系统化方法：
- 版本控制数据库结构
- 团队协作同步
- 生产环境安全部署
- 支持回滚操作

## SQLx 迁移系统

### 安装 SQLx CLI

```bash
# 安装 SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# 或者支持多种数据库
cargo install sqlx-cli
```

### 项目配置

```toml
# Cargo.toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "migrate"] }
tokio = { version = "1", features = ["full"] }
```

### 创建迁移

```bash
# 创建新迁移
sqlx migrate add create_users_table

# 这会创建文件：migrations/20240116000000_create_users_table.sql
```

### 迁移文件示例

```sql
-- migrations/20240116000000_create_users_table.sql
-- 创建用户表
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);

-- 创建更新时间触发器
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

### 运行迁移

```bash
# 设置数据库 URL
export DATABASE_URL="postgres://username:password@localhost/dbname"

# 运行所有待执行的迁移
sqlx migrate run

# 查看迁移状态
sqlx migrate info

# 回滚最后一次迁移
sqlx migrate revert
```

## 迁移最佳实践

### 1. 向前兼容的迁移

```sql
-- ✅ 好：添加可空列
ALTER TABLE users ADD COLUMN phone VARCHAR(20);

-- ❌ 不好：添加非空列（会导致现有数据失败）
ALTER TABLE users ADD COLUMN phone VARCHAR(20) NOT NULL;

-- ✅ 好：分两步添加非空列
-- 第一步：添加可空列
ALTER TABLE users ADD COLUMN phone VARCHAR(20);

-- 第二步：填充数据后设置非空
-- migrations/20240116000002_make_phone_not_null.sql
UPDATE users SET phone = '' WHERE phone IS NULL;
ALTER TABLE users ALTER COLUMN phone SET NOT NULL;
```

### 2. 重命名列

```sql
-- migrations/20240116000003_rename_username_to_user_name.sql
ALTER TABLE users RENAME COLUMN username TO user_name;
```

### 3. 修改列类型

```sql
-- migrations/20240116000004_change_email_length.sql
ALTER TABLE users ALTER COLUMN email TYPE VARCHAR(255);
```

### 4. 添加外键

```sql
-- migrations/20240116000005_create_posts_table.sql
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_posts_user_id ON posts(user_id);
```

## 复杂迁移示例

### 多对多关系

```sql
-- migrations/20240116000006_create_tags_and_post_tags.sql
-- 标签表
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 文章标签关联表
CREATE TABLE post_tags (
    post_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE INDEX idx_post_tags_post_id ON post_tags(post_id);
CREATE INDEX idx_post_tags_tag_id ON post_tags(tag_id);
```

### 数据迁移

```sql
-- migrations/20240116000007_migrate_user_data.sql
-- 假设要将旧的 full_name 拆分为 first_name 和 last_name

-- 添加新列
ALTER TABLE users ADD COLUMN first_name VARCHAR(50);
ALTER TABLE users ADD COLUMN last_name VARCHAR(50);

-- 迁移数据
UPDATE users
SET 
    first_name = SPLIT_PART(full_name, ' ', 1),
    last_name = SPLIT_PART(full_name, ' ', 2)
WHERE full_name IS NOT NULL;

-- 删除旧列（可选，可以在后续迁移中删除）
-- ALTER TABLE users DROP COLUMN full_name;
```

### 添加枚举类型

```sql
-- migrations/20240116000008_add_user_role.sql
-- 创建枚举类型
CREATE TYPE user_role AS ENUM ('admin', 'editor', 'user');

-- 添加角色列
ALTER TABLE users ADD COLUMN role user_role DEFAULT 'user' NOT NULL;

-- 创建索引
CREATE INDEX idx_users_role ON users(role);
```

## 在代码中运行迁移

### 启动时自动迁移

```rust
use sqlx::postgres::PgPoolOptions;
use sqlx::migrate::MigrateDatabase;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
    
    // 如果数据库不存在，创建它
    if !sqlx::Postgres::database_exists(&database_url).await? {
        println!("Creating database...");
        sqlx::Postgres::create_database(&database_url).await?;
    }
    
    // 连接数据库
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // 运行迁移
    println!("Running migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    println!("Migrations completed successfully!");
    
    Ok(())
}
```

### 条件迁移

```rust
use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!();

async fn run_migrations_if_needed(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 检查是否需要迁移
    let version = MIGRATOR.migrations.last().map(|m| m.version).unwrap_or(0);
    
    println!("Latest migration version: {}", version);
    
    // 运行迁移
    MIGRATOR.run(pool).await?;
    
    Ok(())
}
```

## 迁移测试

### 测试迁移

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    
    #[tokio::test]
    async fn test_migrations() {
        let database_url = "postgres://localhost/test_db";
        
        // 创建测试数据库
        sqlx::Postgres::create_database(database_url)
            .await
            .ok();
        
        let pool = PgPoolOptions::new()
            .connect(database_url)
            .await
            .unwrap();
        
        // 运行迁移
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .unwrap();
        
        // 验证表存在
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = 'users'"
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        
        assert_eq!(result.0, 1);
        
        // 清理
        sqlx::Postgres::drop_database(database_url)
            .await
            .ok();
    }
}
```

## 回滚策略

### 创建可回滚的迁移

```sql
-- migrations/20240116000009_add_user_bio.up.sql
ALTER TABLE users ADD COLUMN bio TEXT;

-- migrations/20240116000009_add_user_bio.down.sql
ALTER TABLE users DROP COLUMN bio;
```

### 手动回滚

```bash
# 回滚最后一次迁移
sqlx migrate revert

# 回滚到特定版本
sqlx migrate revert --target-version 20240116000005
```

## 生产环境迁移

### 迁移脚本

```bash
#!/bin/bash
# deploy-migrations.sh

set -e

echo "Starting database migration..."

# 备份数据库
pg_dump $DATABASE_URL > backup_$(date +%Y%m%d_%H%M%S).sql

# 运行迁移
sqlx migrate run

echo "Migration completed successfully!"
```

### 零停机迁移

```sql
-- 步骤 1: 添加新列（可空）
ALTER TABLE users ADD COLUMN new_email VARCHAR(255);

-- 步骤 2: 双写（应用代码同时写入两列）
-- 在应用代码中实现

-- 步骤 3: 数据回填
UPDATE users SET new_email = email WHERE new_email IS NULL;

-- 步骤 4: 设置非空约束
ALTER TABLE users ALTER COLUMN new_email SET NOT NULL;

-- 步骤 5: 添加唯一约束
ALTER TABLE users ADD CONSTRAINT users_new_email_key UNIQUE (new_email);

-- 步骤 6: 删除旧列（在确认无问题后）
ALTER TABLE users DROP COLUMN email;

-- 步骤 7: 重命名新列
ALTER TABLE users RENAME COLUMN new_email TO email;
```

## 完整示例项目

```rust
// src/main.rs
use sqlx::postgres::PgPoolOptions;
use std::env;

mod models;
mod migrations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    // 创建连接池
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // 运行迁移
    println!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    println!("Migrations completed!");
    
    // 启动应用
    start_server(pool).await?;
    
    Ok(())
}

async fn start_server(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // 应用逻辑
    Ok(())
}
```

```sql
-- migrations/20240116000001_initial_schema.sql
-- 用户表
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(20) DEFAULT 'user' NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 文章表
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL,
    slug VARCHAR(200) UNIQUE NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 评论表
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 索引
CREATE INDEX idx_posts_user_id ON posts(user_id);
CREATE INDEX idx_posts_slug ON posts(slug);
CREATE INDEX idx_posts_published ON posts(published);
CREATE INDEX idx_comments_post_id ON comments(post_id);
CREATE INDEX idx_comments_user_id ON comments(user_id);

-- 更新时间触发器
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_posts_updated_at
    BEFORE UPDATE ON posts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

## 迁移检查清单

- [ ] 迁移文件命名清晰
- [ ] 包含向前和向后兼容性
- [ ] 测试迁移在干净数据库上运行
- [ ] 测试迁移在现有数据上运行
- [ ] 准备回滚计划
- [ ] 在生产前备份数据库
- [ ] 文档化重要变更
- [ ] 通知团队成员

## 练习

1. 创建完整的博客数据库 schema
2. 实现用户角色系统的迁移
3. 添加全文搜索索引
4. 实现数据归档迁移
5. 创建迁移测试套件

## 下一步

Day 67 将学习 CRUD 操作实战，使用 SQLx 进行数据库操作。
