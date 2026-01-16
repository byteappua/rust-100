# Day 68: 复杂查询与事务

## 学习目标
- 掌握 JOIN 查询
- 学习聚合函数
- 理解事务管理
- 实现复杂业务逻辑

## JOIN 查询

### INNER JOIN

```rust
#[derive(Debug, Serialize, FromRow)]
struct PostWithAuthor {
    post_id: i32,
    title: String,
    content: String,
    author_id: i32,
    author_name: String,
    author_email: String,
}

async fn get_posts_with_authors(pool: &PgPool) -> Result<Vec<PostWithAuthor>, sqlx::Error> {
    let posts = sqlx::query_as!(
        PostWithAuthor,
        r#"
        SELECT 
            p.id as post_id,
            p.title,
            p.content,
            u.id as author_id,
            u.username as author_name,
            u.email as author_email
        FROM posts p
        INNER JOIN users u ON p.user_id = u.id
        ORDER BY p.created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;
    
    Ok(posts)
}
```

### LEFT JOIN

```rust
#[derive(Debug, Serialize, FromRow)]
struct UserWithPostCount {
    user_id: i32,
    username: String,
    post_count: Option<i64>,
}

async fn get_users_with_post_count(pool: &PgPool) -> Result<Vec<UserWithPostCount>, sqlx::Error> {
    let users = sqlx::query_as!(
        UserWithPostCount,
        r#"
        SELECT 
            u.id as user_id,
            u.username,
            COUNT(p.id) as "post_count?"
        FROM users u
        LEFT JOIN posts p ON u.id = p.user_id
        GROUP BY u.id, u.username
        ORDER BY post_count DESC
        "#
    )
    .fetch_all(pool)
    .await?;
    
    Ok(users)
}
```

## 聚合查询

### 统计函数

```rust
#[derive(Debug, Serialize, FromRow)]
struct PostStatistics {
    total_posts: i64,
    published_posts: i64,
    draft_posts: i64,
    avg_content_length: f64,
}

async fn get_post_statistics(pool: &PgPool) -> Result<PostStatistics, sqlx::Error> {
    let stats = sqlx::query_as!(
        PostStatistics,
        r#"
        SELECT 
            COUNT(*) as "total_posts!",
            COUNT(*) FILTER (WHERE published = true) as "published_posts!",
            COUNT(*) FILTER (WHERE published = false) as "draft_posts!",
            AVG(LENGTH(content))::float as "avg_content_length!"
        FROM posts
        "#
    )
    .fetch_one(pool)
    .await?;
    
    Ok(stats)
}
```

## 事务管理

### 基本事务

```rust
async fn transfer_points(
    pool: &PgPool,
    from_user: i32,
    to_user: i32,
    amount: i32
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    
    // 扣除发送方积分
    sqlx::query!(
        "UPDATE users SET points = points - $1 WHERE id = $2",
        amount,
        from_user
    )
    .execute(&mut *tx)
    .await?;
    
    // 增加接收方积分
    sqlx::query!(
        "UPDATE users SET points = points + $1 WHERE id = $2",
        amount,
        to_user
    )
    .execute(&mut *tx)
    .await?;
    
    // 记录转账
    sqlx::query!(
        "INSERT INTO point_transfers (from_user, to_user, amount) VALUES ($1, $2, $3)",
        from_user,
        to_user,
        amount
    )
    .execute(&mut *tx)
    .await?;
    
    tx.commit().await?;
    
    Ok(())
}
```

### 事务回滚

```rust
async fn create_post_with_tags(
    pool: &PgPool,
    post: CreatePost,
    tag_names: Vec<String>
) -> Result<Post, sqlx::Error> {
    let mut tx = pool.begin().await?;
    
    // 创建文章
    let post = sqlx::query_as!(
        Post,
        r#"
        INSERT INTO posts (user_id, title, content)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, title, content, created_at
        "#,
        post.user_id,
        post.title,
        post.content
    )
    .fetch_one(&mut *tx)
    .await?;
    
    // 处理标签
    for tag_name in tag_names {
        // 获取或创建标签
        let tag = sqlx::query_as!(
            Tag,
            r#"
            INSERT INTO tags (name)
            VALUES ($1)
            ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
            RETURNING id, name
            "#,
            tag_name
        )
        .fetch_one(&mut *tx)
        .await?;
        
        // 关联文章和标签
        sqlx::query!(
            "INSERT INTO post_tags (post_id, tag_id) VALUES ($1, $2)",
            post.id,
            tag.id
        )
        .execute(&mut *tx)
        .await?;
    }
    
    tx.commit().await?;
    
    Ok(post)
}
```

## 子查询

```rust
async fn get_popular_posts(pool: &PgPool, limit: i64) -> Result<Vec<Post>, sqlx::Error> {
    let posts = sqlx::query_as!(
        Post,
        r#"
        SELECT p.*
        FROM posts p
        WHERE p.id IN (
            SELECT post_id
            FROM comments
            GROUP BY post_id
            HAVING COUNT(*) > 10
        )
        ORDER BY p.created_at DESC
        LIMIT $1
        "#,
        limit
    )
    .fetch_all(pool)
    .await?;
    
    Ok(posts)
}
```

## 完整示例

详见 Day61-80/STAGE5_OVERVIEW.md 中的复杂查询部分。

## 练习

1. 实现多表关联查询
2. 创建复杂的统计报表
3. 实现事务性的订单系统
4. 优化慢查询
5. 实现数据导入导出

## 下一步

Day 69 已完成 - JWT 用户认证系统。
