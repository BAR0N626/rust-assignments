use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub id: i64,
    pub title: String,
    pub code: String,
    pub created_at: i64,
}

impl Snippet {
    pub fn new(id: i64, title: String, code: String) -> Result<Self> {
        let created_at = current_unix_time().context("Failed to get snippet creation time")?;
        Ok(Self {
            id,
            title,
            code,
            created_at,
        })
    }
}

fn current_unix_time() -> Result<i64> {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("System time is before UNIX_EPOCH")?
        .as_secs();

    i64::try_from(secs).context("Failed to convert UNIX time to i64")
}
