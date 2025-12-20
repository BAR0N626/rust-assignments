use serde::Deserialize;
use std::env;

use clap::Parser;
use config::{Config as RawConfig, Environment, File};

#[derive(Parser, Debug)]
#[command(name = "task_3_9")]
struct Cli {
    /// Enables debug mode
    #[arg(short, long)]
    debug: bool,

    /// Path to configuration file
    #[arg(short, long, default_value = "config.toml", env = "CONF_FILE")]
    conf: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    mode: Mode,
    server: Server,
    db: Db,
    log: Log,
    background: Background,
}

#[derive(Debug, Deserialize)]
struct Mode {
    debug: bool,
}

#[derive(Debug, Deserialize)]
struct Server {
    external_url: String,
    http_port: u16,
    grpc_port: u16,
    healthz_port: u16,
    metrics_port: u16,
}

#[derive(Debug, Deserialize)]
struct Db {
    mysql: Mysql,
}

#[derive(Debug, Deserialize)]
struct Mysql {
    host: String,
    port: u16,
    dating: String,
    user: String,
    pass: String,
    connections: MysqlConnections,
}

#[derive(Debug, Deserialize)]
struct MysqlConnections {
    max_idle: u32,
    max_open: u32,
}

#[derive(Debug, Deserialize)]
struct Log {
    app: LogApp,
}

#[derive(Debug, Deserialize)]
struct LogApp {
    level: String,
}

#[derive(Debug, Deserialize)]
struct Background {
    watchdog: Watchdog,
}

#[derive(Debug, Deserialize)]
struct Watchdog {
    period: String,
    limit: u32,
    lock_timeout: String,
}

fn load_config(cli: &Cli) -> Result<Config, config::ConfigError> {
    let mut builder = RawConfig::builder()
        // defaults
        .set_default("mode.debug", false)?
        .set_default("server.external_url", "http://127.0.0.1")?
        .set_default("server.http_port", 8081)?
        .set_default("server.grpc_port", 8082)?
        .set_default("server.healthz_port", 10025)?
        .set_default("server.metrics_port", 9199)?
        .set_default("db.mysql.host", "127.0.0.1")?
        .set_default("db.mysql.port", 3306)?
        .set_default("db.mysql.dating", "default")?
        .set_default("db.mysql.user", "root")?
        .set_default("db.mysql.pass", "")?
        .set_default("db.mysql.connections.max_idle", 30)?
        .set_default("db.mysql.connections.max_open", 30)?
        .set_default("log.app.level", "info")?
        .set_default("background.watchdog.period", "5s")?
        .set_default("background.watchdog.limit", 10)?
        .set_default("background.watchdog.lock_timeout", "4s")?
        // file
        .add_source(File::with_name(&cli.conf).required(false))
        // env CONF__...
        .add_source(Environment::with_prefix("CONF").separator("__").try_parsing(true));

    // CLI override has highest priority
    if cli.debug {
        builder = builder.set_override("mode.debug", true)?;
    }

    let raw = builder.build()?;
    raw.try_deserialize::<Config>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let cfg = load_config(&cli)?;
    println!("{:#?}", cfg);
    Ok(())
}
