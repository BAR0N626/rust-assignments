use anyhow::{Context, Result};
use std::{fs, path::PathBuf};

use crate::model::Snippet;
use crate::storage::SnippetStorage;

pub struct JsonStorage {
    path: PathBuf,
}

impl JsonStorage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn read_all(&self) -> Result<Vec<Snippet>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.path)
            .with_context(|| format!("Failed to read JSON file: {:?}", self.path))?;

        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

        let snippets: Vec<Snippet> =
            serde_json::from_str(&content).context("Failed to parse JSON file")?;
        Ok(snippets)
    }

    fn write_all(&self, snippets: &[Snippet]) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory {:?}", parent))?;
            }
        }

        let json = serde_json::to_string_pretty(snippets)
            .context("Failed to serialize snippets to JSON")?;

        fs::write(&self.path, json)
            .with_context(|| format!("Failed to write JSON file: {:?}", self.path))?;

        Ok(())
    }
}

impl SnippetStorage for JsonStorage {
    fn save(&mut self, snippet: Snippet) -> Result<()> {
        let mut all = self.read_all().context("Failed to read existing snippets")?;

        // upsert по title
        if let Some(pos) = all.iter().position(|s| s.title == snippet.title) {
            all[pos] = snippet;
        } else {
            all.push(snippet);
        }

        self.write_all(&all).context("Failed to write updated snippets")?;
        Ok(())
    }

    fn get_by_title(&self, title: &str) -> Result<Option<Snippet>> {
        let all = self.read_all()?;
        Ok(all.into_iter().find(|s| s.title == title))
    }

    fn delete_by_title(&mut self, title: &str) -> Result<bool> {
        let mut all = self.read_all()?;
        let before = all.len();
        all.retain(|s| s.title != title);
        let deleted = all.len() != before;
        self.write_all(&all)?;
        Ok(deleted)
    }

    fn list(&self) -> Result<Vec<Snippet>> {
        let mut all = self.read_all()?;
        all.sort_by_key(|s| s.id);
        Ok(all)
    }

    fn next_id(&self) -> Result<i64> {
        let all = self.read_all()?;
        let max_id = all.iter().map(|s| s.id).max().unwrap_or(0);
        Ok(max_id + 1)
    }
}
