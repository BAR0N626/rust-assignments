use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::{fs, path::Path};

use crate::model::Snippet;
use crate::storage::SnippetStorage;

pub struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    pub fn new(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory {:?}", parent))?;
            }
        }

        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open SQLite DB at {:?}", path))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS snippets (
                id         INTEGER PRIMARY KEY,
                title      TEXT NOT NULL UNIQUE,
                code       TEXT NOT NULL,
                created_at INTEGER NOT NULL
            )",
            [],
        )
        .context("Failed to create SQLite table")?;

        Ok(Self { conn })
    }
}

impl SnippetStorage for SqliteStorage {
    fn save(&mut self, snippet: Snippet) -> Result<()> {
        // upsert по title (логичнее для snippets-app)
        self.conn
            .execute(
                "INSERT INTO snippets (id, title, code, created_at)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(title) DO UPDATE SET
                    code = excluded.code,
                    created_at = excluded.created_at;",
                params![snippet.id, snippet.title, snippet.code, snippet.created_at],
            )
            .context("Failed to insert/update snippet in SQLite DB")?;
        Ok(())
    }

    fn get_by_title(&self, title: &str) -> Result<Option<Snippet>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, code, created_at FROM snippets WHERE title = ?1")
            .context("Failed to prepare SELECT")?;

        let mut rows = stmt.query(params![title]).context("Failed to query")?;

        if let Some(row) = rows.next().context("Failed to read row")? {
            Ok(Some(Snippet {
                id: row.get(0)?,
                title: row.get(1)?,
                code: row.get(2)?,
                created_at: row.get(3)?,
            }))
        } else {
            Ok(None)
        }
    }

    fn delete_by_title(&mut self, title: &str) -> Result<bool> {
        let affected = self
            .conn
            .execute("DELETE FROM snippets WHERE title = ?1", params![title])
            .context("Failed to delete snippet")?;
        Ok(affected > 0)
    }

    fn list(&self) -> Result<Vec<Snippet>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, code, created_at FROM snippets ORDER BY id ASC")
            .context("Failed to prepare SELECT")?;

        let rows = stmt
            .query_map([], |row| {
                Ok(Snippet {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    code: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })
            .context("Failed to query snippets")?;

        let mut out = Vec::new();
        for r in rows {
            out.push(r.context("Failed to read SQLite row")?);
        }
        Ok(out)
    }

    fn next_id(&self) -> Result<i64> {
        let mut stmt = self
            .conn
            .prepare("SELECT COALESCE(MAX(id), 0) + 1 FROM snippets")
            .context("Failed to prepare MAX(id)")?;

        let next: i64 = stmt.query_row([], |row| row.get(0)).context("Failed to read MAX(id)")?;
        Ok(next)
    }
}
