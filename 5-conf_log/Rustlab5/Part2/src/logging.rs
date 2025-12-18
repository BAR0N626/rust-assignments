use anyhow::{Context, Result};
use std::{env, fs, fs::OpenOptions, path::Path};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_logging() -> Result<WorkerGuard> {
    let level = env::var("SNIPPETS_APP_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let path = env::var("SNIPPETS_APP_LOG_PATH").unwrap_or_else(|_| "snippets-app.log".to_string());

    // ✅ создаём папку под лог-файл, если её нет
    let p = Path::new(&path);
    if let Some(parent) = p.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create log directory {:?}", parent))?;
        }
    }

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .with_context(|| format!("Failed to open log file at {path}"))?;

    let (nb, guard) = tracing_appender::non_blocking(file);

    let filter = EnvFilter::try_new(level).context("Invalid SNIPPETS_APP_LOG_LEVEL")?;

    fmt()
        .with_env_filter(filter)
        .with_writer(nb)
        .with_target(false)
        .with_ansi(false)
        .init();

    Ok(guard)
}
