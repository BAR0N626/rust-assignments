use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Read};

mod logging;
mod model;
mod storage;

use model::Snippet;
use storage::{storage_from_env, SnippetStorage};

#[derive(Parser, Debug)]
#[command(name = "snippets-app", version, about = "Store and read code snippets")]
struct Cli {
    /// Create snippet with this name (reads from stdin by default)
    #[arg(long)]
    name: Option<String>,

    /// Read snippet by name
    #[arg(long)]
    read: Option<String>,

    /// Delete snippet by name
    #[arg(long)]
    delete: Option<String>,

    /// Optional: download snippet body from URL instead of stdin (works with --name)
    #[arg(long)]
    download: Option<String>,
}

fn main() -> Result<()> {
    let _log_guard = logging::init_logging().context("Failed to init logging")?;
    let cli = Cli::parse();

    tracing::info!("Starting snippets-app");

    let mut storage = storage_from_env().context("Failed to initialize storage backend")?;
    tracing::debug!("Storage backend initialized");

    // Priority of actions:
    // 1) create (--name)
    // 2) read (--read)
    // 3) delete (--delete)
    // 4) list (default)
    if let Some(name) = cli.name {
        let code = if let Some(url) = cli.download {
            tracing::info!(%url, "Downloading snippet body");
            download_text(&url).context("Failed to download snippet")?
        } else {
            tracing::info!("Reading snippet body from stdin");
            read_stdin().context("Failed to read stdin")?
        };

        let id = storage.next_id().context("Failed to compute next id")?;
        let snippet = Snippet::new(id, name.clone(), code).context("Failed to create snippet")?;

        storage.save(snippet).context("Failed to save snippet")?;
        tracing::info!(title=%name, "Snippet saved");
        println!("OK: saved snippet \"{name}\"");
        return Ok(());
    }

    if let Some(name) = cli.read {
        tracing::info!(title=%name, "Reading snippet");
        match storage.get_by_title(&name).context("Failed to read from storage")? {
            Some(s) => {
                println!("{}", s.code);
                return Ok(());
            }
            None => {
                println!("Not found: \"{name}\"");
                return Ok(());
            }
        }
    }

    if let Some(name) = cli.delete {
        tracing::info!(title=%name, "Deleting snippet");
        let deleted = storage
            .delete_by_title(&name)
            .context("Failed to delete from storage")?;

        if deleted {
            println!("OK: deleted \"{name}\"");
        } else {
            println!("Not found: \"{name}\"");
        }
        return Ok(());
    }

    tracing::info!("Listing snippets");
    let snippets = storage.list().context("Failed to list snippets")?;
    if snippets.is_empty() {
        println!("No snippets stored.");
    } else {
        println!("Stored snippets:");
        for s in snippets {
            println!("- {} (id={}, created_at={})", s.title, s.id, s.created_at);
        }
    }

    Ok(())
}

fn read_stdin() -> Result<String> {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .context("Failed to read stdin to string")?;
    Ok(buf)
}

fn download_text(url: &str) -> Result<String> {
    let resp = reqwest::blocking::get(url)
        .with_context(|| format!("GET request failed: {url}"))?;

    let status = resp.status();
    if !status.is_success() {
        anyhow::bail!("Download failed with status {status}");
    }

    let text = resp.text().context("Failed to read response body as text")?;
    Ok(text)
}
