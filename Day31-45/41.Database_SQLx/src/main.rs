use sqlx::sqlite::SqlitePoolOptions;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("--- Day 41: Database Interaction with SQLx ---");

    // 1. Create a connection pool (using in-memory SQLite for demo)
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await?;

    // 2. Create table
    sqlx::query(
        r#"
        CREATE TABLE todos (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT 0
        )
        "#,
    )
    .execute(&pool)
    .await?;
    println!("Created table 'todos'.");

    // 3. Insert data
    let description = "Learn SQLx";
    let id: i64 = sqlx::query("INSERT INTO todos (description) VALUES (?)")
        .bind(description)
        .execute(&pool)
        .await?
        .last_insert_rowid();

    println!("Inserted todo with id: {}", id);

    // 4. Query data (Map to struct)
    let todos: Vec<Todo> = sqlx::query_as("SELECT id, description, done FROM todos")
        .fetch_all(&pool)
        .await?;

    println!("Query result:");
    for todo in todos {
        println!("  - [{}] {} (Done: {})", todo.id, todo.description, todo.done);
    }

    // 5. Update data
    sqlx::query("UPDATE todos SET done = ? WHERE id = ?")
        .bind(true)
        .bind(id)
        .execute(&pool)
        .await?;
    println!("Marked todo {} as done.", id);

    Ok(())
}
