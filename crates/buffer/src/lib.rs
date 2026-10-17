use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
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
        let config = BufferConfig {
            path: path.to_string(),
            ..Default::default()
        };
        Self::with_config(config).await
    }

    pub async fn with_config(config: BufferConfig) -> Result<Self, BufferError> {
        // Applied per connection, so every pooled connection gets WAL + NORMAL sync.
        let options = connect_options(&config.path)?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);
        let pool = SqlitePoolOptions::new()
            .max_connections(config.max_connections)
            .connect_with(options)
            .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS telemetry_queue (\
                id INTEGER PRIMARY KEY AUTOINCREMENT,\
                topic TEXT NOT NULL,\
                payload BLOB NOT NULL,\
                created_at INTEGER NOT NULL\
            )",
        )
        .execute(&pool)
        .await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_created_at ON telemetry_queue(created_at)")
            .execute(&pool)
            .await?;

        info!(path = %config.path, max_connections = config.max_connections, "buffer initialized");

        Ok(Self { pool })
    }

    pub async fn enqueue(&self, topic: &str, payload: &[u8]) -> Result<(), BufferError> {
        sqlx::query("INSERT INTO telemetry_queue (topic, payload, created_at) VALUES (?, ?, ?)")
            .bind(topic)
            .bind(payload)
            .bind(unix_ms())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Inserts many payloads in one transaction: one fsync instead of one per row,
    /// which matters on flash storage.
    pub async fn enqueue_batch<P: AsRef<[u8]>>(
        &self,
        topic: &str,
        payloads: &[P],
    ) -> Result<(), BufferError> {
        if payloads.is_empty() {
            return Ok(());
        }
        let created_at = unix_ms();
        let mut tx = self.pool.begin().await?;
        for payload in payloads {
            sqlx::query(
                "INSERT INTO telemetry_queue (topic, payload, created_at) VALUES (?, ?, ?)",
            )
            .bind(topic)
            .bind(payload.as_ref())
            .bind(created_at)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// Deletes the oldest rows so at most `max_rows` remain. Returns rows deleted.
    pub async fn trim_to_max_rows(&self, max_rows: i64) -> Result<u64, BufferError> {
        let result = sqlx::query(
            "DELETE FROM telemetry_queue WHERE id <= \
             (SELECT id FROM telemetry_queue ORDER BY id DESC LIMIT 1 OFFSET ?)",
        )
        .bind(max_rows)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    /// Deletes rows enqueued before `cutoff_unix_ms`. Returns rows deleted.
    pub async fn delete_older_than(&self, cutoff_unix_ms: i64) -> Result<u64, BufferError> {
        let result = sqlx::query("DELETE FROM telemetry_queue WHERE created_at < ?")
            .bind(cutoff_unix_ms)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// Enqueue time (Unix ms) of the oldest buffered row, if any.
    pub async fn oldest_created_at(&self) -> Result<Option<i64>, BufferError> {
        let row = sqlx::query("SELECT MIN(created_at) AS oldest FROM telemetry_queue")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.get::<Option<i64>, _>("oldest"))
    }

    pub async fn dequeue_batch(&self, limit: i64) -> Result<Vec<BufferedMessage>, BufferError> {
        let rows =
            sqlx::query("SELECT id, topic, payload FROM telemetry_queue ORDER BY id ASC LIMIT ?")
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

fn connect_options(path: &str) -> Result<SqliteConnectOptions, BufferError> {
    if path.starts_with("sqlite:") {
        Ok(SqliteConnectOptions::from_str(path)?)
    } else {
        Ok(SqliteConnectOptions::new().filename(path))
    }
}

fn unix_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
