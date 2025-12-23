#![allow(dead_code)]

use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Row, SqlitePool};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Clone)]
pub struct BufferStore {
    pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct BufferConfig {
    pub path: String,
    pub max_connections: u32,
}

impl Default for BufferConfig {
    fn default() -> Self {
        Self {
            path: "sunspec-buffer.sqlite".to_string(),
            max_connections: 5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BufferedMessage {
    pub id: i64,
    pub topic: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum BufferError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl BufferStore {
    pub async fn new(path: &str) -> Result<Self, BufferError> {
        let url = sqlite_url(path);
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await?;

        sqlx::query("PRAGMA journal_mode = WAL;")
            .execute(&pool)
            .await?;
        sqlx::query("PRAGMA synchronous = NORMAL;")
            .execute(&pool)
            .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS telemetry_queue (\
                id INTEGER PRIMARY KEY AUTOINCREMENT,\
                topic TEXT NOT NULL,\
                payload BLOB NOT NULL,\
                retry_count INTEGER DEFAULT 0,\
                created_at INTEGER NOT NULL\
            )",
        )
        .execute(&pool)
        .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_created_at ON telemetry_queue(created_at)",
        )
        .execute(&pool)
        .await?;

        info!(path = %path, "buffer initialized");

        Ok(Self { pool })
    }

    pub async fn enqueue(&self, topic: &str, payload: &[u8]) -> Result<(), BufferError> {
        sqlx::query(
            "INSERT INTO telemetry_queue (topic, payload, created_at) VALUES (?, ?, ?)",
        )
        .bind(topic)
        .bind(payload)
        .bind(unix_ms())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn dequeue_batch(&self, limit: i64) -> Result<Vec<BufferedMessage>, BufferError> {
        let rows = sqlx::query(
            "SELECT id, topic, payload FROM telemetry_queue ORDER BY id ASC LIMIT ?",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let messages = rows
            .into_iter()
            .map(|row| BufferedMessage {
                id: row.get::<i64, _>("id"),
                topic: row.get::<String, _>("topic"),
                payload: row.get::<Vec<u8>, _>("payload"),
            })
            .collect();

        Ok(messages)
    }

    pub async fn delete_batch(&self, ids: &[i64]) -> Result<(), BufferError> {
        if ids.is_empty() {
            return Ok(());
        }

        let mut query = String::from("DELETE FROM telemetry_queue WHERE id IN (");
        for (idx, _) in ids.iter().enumerate() {
            if idx > 0 {
                query.push_str(", ");
            }
            query.push('?');
        }
        query.push(')');

        let mut statement = sqlx::query(&query);
        for id in ids {
            statement = statement.bind(id);
        }
        statement.execute(&self.pool).await?;

        Ok(())
    }

    pub async fn pending_count(&self) -> Result<i64, BufferError> {
        let row = sqlx::query("SELECT COUNT(*) AS count FROM telemetry_queue")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.get::<i64, _>("count"))
    }
}

fn sqlite_url(path: &str) -> String {
    if path.starts_with("sqlite:") {
        path.to_string()
    } else {
        format!("sqlite://{path}")
    }
}

fn unix_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
