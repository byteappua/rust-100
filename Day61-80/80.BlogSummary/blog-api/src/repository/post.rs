use crate::models::Post;
use anyhow::Result;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct PostRepository {
    pool: SqlitePool,
}

impl PostRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        title: &str,
        slug: &str,
        content: &str,
        author_id: i64,
    ) -> Result<Post> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            INSERT INTO posts (title, slug, content, author_id, published)
            VALUES ($1, $2, $3, $4, FALSE)
            RETURNING id, title, slug, content, author_id, published, created_at, updated_at
            "#,
        )
        .bind(title)
        .bind(slug)
        .bind(content)
        .bind(author_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    pub async fn find_all(&self) -> Result<Vec<Post>> {
        let posts = sqlx::query_as::<_, Post>(
            "SELECT id, title, slug, content, author_id, published, created_at, updated_at FROM posts ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Post>> {
        let post = sqlx::query_as::<_, Post>(
            "SELECT id, title, slug, content, author_id, published, created_at, updated_at FROM posts WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    pub async fn update(
        &self,
        id: i64,
        title: &str,
        slug: &str,
        content: &str,
        published: bool,
    ) -> Result<Option<Post>> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            UPDATE posts
            SET title = $1, slug = $2, content = $3, published = $4, updated_at = CURRENT_TIMESTAMP
            WHERE id = $5
            RETURNING id, title, slug, content, author_id, published, created_at, updated_at
            "#,
        )
        .bind(title)
        .bind(slug)
        .bind(content)
        .bind(published)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM posts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
