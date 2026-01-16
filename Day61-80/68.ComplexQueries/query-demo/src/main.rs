use sqlx::{SqlitePool, sqlite::SqlitePoolOptions, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i64,
    username: String,
    points: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserPostCount {
    username: String,
    post_count: i64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Setup DB (In-memory)
    let db_url = "sqlite::memory:";
    let pool = SqlitePoolOptions::new().connect(db_url).await?;

    // 2. Initialize Schema and Data
    init_db(&pool).await?;

    println!("--- Initial State ---");
    list_users(&pool).await?;

    // 3. Complex Query: JOIN and Aggregation
    println!("\n--- Users with Post Counts (JOIN + GROUP BY) ---");
    let counts = get_users_with_post_count(&pool).await?;
    for c in counts {
        println!("{:?}", c);
    }

    // 4. Transaction: Transfer Points
    println!("\n--- Transferring Points (Transaction) ---");
    match transfer_points(&pool, 1, 2, 50).await {
        Ok(_) => println!("Transfer successful!"),
        Err(e) => println!("Transfer failed: {}", e),
    }

    println!("\n--- Final State ---");
    list_users(&pool).await?;

    Ok(())
}

async fn init_db(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, username TEXT, points INTEGER)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE TABLE posts (id INTEGER PRIMARY KEY, user_id INTEGER, title TEXT)")
        .execute(pool)
        .await?;

    // Seed Users
    sqlx::query("INSERT INTO users (id, username, points) VALUES (1, 'Alice', 100), (2, 'Bob', 50)")
        .execute(pool)
        .await?;

    // Seed Posts
    sqlx::query("INSERT INTO posts (user_id, title) VALUES (1, 'Alice Post 1'), (1, 'Alice Post 2'), (2, 'Bob Post 1')")
        .execute(pool)
        .await?;

    Ok(())
}

async fn list_users(pool: &SqlitePool) -> anyhow::Result<()> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await?;
    for user in users {
        println!("{:?}", user);
    }
    Ok(())
}

// JOIN and Aggregation
async fn get_users_with_post_count(pool: &SqlitePool) -> anyhow::Result<Vec<UserPostCount>> {
    let rows = sqlx::query(
        r#"
        SELECT u.username, COUNT(p.id) as post_count
        FROM users u
        LEFT JOIN posts p ON u.id = p.user_id
        GROUP BY u.id
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for row in rows {
        result.push(UserPostCount {
            username: row.try_get("username")?,
            post_count: row.try_get("post_count")?,
        });
    }
    Ok(result)
}

// Transaction
async fn transfer_points(pool: &SqlitePool, from_id: i64, to_id: i64, amount: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    // Deduct
    sqlx::query("UPDATE users SET points = points - $1 WHERE id = $2")
        .bind(amount)
        .bind(from_id)
        .execute(&mut *tx)
        .await?;

    // Add
    sqlx::query("UPDATE users SET points = points + $1 WHERE id = $2")
        .bind(amount)
        .bind(to_id)
        .execute(&mut *tx)
        .await?;

    // Commit
    tx.commit().await?;
    Ok(())
}
