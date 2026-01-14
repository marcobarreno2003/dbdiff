use anyhow::Result;
use crate::schema::models::Schema;

/// Storage manager for local SQLite database
pub struct Storage {
    // TODO: SQLite connection pool
}

impl Storage {
    pub async fn new(_path: &str) -> Result<Self> {
        // TODO: Initialize SQLite connection
        // TODO: Run migrations
        Ok(Self {})
    }

    pub async fn save_snapshot(&self, _name: &str, _schema: &Schema) -> Result<i64> {
        // TODO: Serialize schema to JSON
        // TODO: Insert into snapshots table
        Ok(0)
    }

    pub async fn load_snapshot(&self, _id: i64) -> Result<Schema> {
        // TODO: Query snapshot by ID
        // TODO: Deserialize JSON to Schema
        Ok(Schema::new())
    }

    pub async fn get_latest_snapshot(&self) -> Result<Option<Schema>> {
        // TODO: Query most recent snapshot
        Ok(None)
    }

    pub async fn list_snapshots(&self, _limit: u32) -> Result<Vec<SnapshotInfo>> {
        // TODO: Query snapshots with metadata
        Ok(Vec::new())
    }
}

#[derive(Debug)]
pub struct SnapshotInfo {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub table_count: usize,
}
