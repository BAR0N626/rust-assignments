use clap::Parser;
use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct Config {
    debug: bool,
    server: ServerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            server: ServerConfig {
                host: "localhost".into(),
                port: 3000,
            },
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[arg(short, long, default_value = "config.toml", env = "CONF_FILE")]
    conf: String,
}

fn main() {
    let cli = Cli::parse();

    // 1. defaults
    let mut config = Config::default();

    // 2. TOML
    if let Ok(toml) = fs::read_to_string(&cli.conf) {
        if let Ok(from_file) = toml::from_str::<Config>(&toml) {
            config = from_file;
        }
    }

    // 3. ENV
    if let Ok(val) = env::var("CONF_DEBUG") {
        config.debug = val == "true";
    }

    if cli.debug {
        config.debug = true;
    }

    println!("{:#?}", config);
}
