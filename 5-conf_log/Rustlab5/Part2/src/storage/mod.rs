use anyhow::Result;
use std::{env, path::{Path, PathBuf}};

use crate::model::Snippet;

pub mod json;
pub mod sqlite;

pub trait SnippetStorage {
    fn save(&mut self, snippet: Snippet) -> Result<()>;
    fn get_by_title(&self, title: &str) -> Result<Option<Snippet>>;
    fn delete_by_title(&mut self, title: &str) -> Result<bool>;
    fn list(&self) -> Result<Vec<Snippet>>;
    fn next_id(&self) -> Result<i64>;
}

pub fn storage_from_env() -> Result<Box<dyn SnippetStorage>> {
    let raw = env::var("SNIPPETS_APP_STORAGE")
        .map_err(|_| anyhow::anyhow!("SNIPPETS_APP_STORAGE is not set (example: JSON:./data/snippets.json or SQLITE:./data/snippets.db)"))?;

    let (kind, path) = raw
        .split_once(':')
        .ok_or_else(|| anyhow::anyhow!("SNIPPETS_APP_STORAGE must be in format PROVIDER:PATH"))?;

    match kind.trim().to_uppercase().as_str() {
        "JSON" => Ok(Box::new(json::JsonStorage::new(PathBuf::from(path)))),
        "SQLITE" => Ok(Box::new(sqlite::SqliteStorage::new(Path::new(path))?)),
        other => Err(anyhow::anyhow!("Unknown provider: {other}. Use JSON or SQLITE.")),
    }
}
