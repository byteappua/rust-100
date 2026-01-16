use sqlx::sqlite::SqlitePoolOptions;
use sqlx::migrate::MigrateDatabase;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Define database URL (using file for SQLite)
    let db_url = "sqlite://demo.db";

    // 2. Create database if not exists
    if !sqlx::Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        sqlx::Sqlite::create_database(db_url).await?;
    } else {
        println!("Database {} already exists", db_url);
    }

    // 3. Connect
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    // 4. Run Migrations
    // This looks for sql files in the ./migrations directory relative to CARGO_MANIFEST_DIR
    println!("Running migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    println!("Migrations completed successfully.");

    // 5. Verify tables exist
    let row: (i64,) = sqlx::query_as("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='users'")
        .fetch_one(&pool)
        .await?;

    if row.0 > 0 {
        println!("Verified: 'users' table exists.");
    }

    Ok(())
}
