use sqlx::PgPool;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn connect(url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(url).await?;
        Ok(Database { pool })
    }

    pub async fn create_user_if_not_exists(&self, username: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (username) VALUES ($1) ON CONFLICT DO NOTHING",
            username
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_message(&self, username: &str, content: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO messages (username, content) VALUES ($1, $2)",
            username,
            content
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_recent_messages(
        &self,
        limit: i64,
    ) -> Result<Vec<(String, String)>, sqlx::Error> {
        let rows = sqlx::query!(
            "SELECT username, content FROM messages
             ORDER BY sent_at DESC
             LIMIT $1",
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        let mut messages: Vec<(String, String)> = rows
            .into_iter()
            .map(|row| (row.username, row.content))
            .collect();

        messages.reverse();
        Ok(messages)
    }
}
