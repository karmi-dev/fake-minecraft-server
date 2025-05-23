mod config;
mod models;
mod protocol;
mod server;

use config::Config;
use log::{debug, info};
use std::{env, io};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yml".to_string());
    let config = match Config::load(config_path) {
        Ok(config) => config,
        Err(e) => {
            panic!("Failed to load config: {} ({:?})", e, e.kind());
        }
    };

    env_logger::init_from_env(env_logger::Env::default().default_filter_or(
        // Set log level to "debug" if enabled in config or environment; default is "info"
        if !config.debug && env::var("DEBUG").map(|v| v != "true").unwrap_or(true) {
            "info"
        } else {
            "debug"
        },
    ));

    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;
    info!("Server listening on port {}", config.port);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                debug!("New connection from: {}", addr);

                let config = config.clone();
                tokio::spawn(async move {
                    if let Err(e) = server::handle_client(stream, config).await {
                        debug!("Error handling client {}: {}", addr, e);
                    }
                });
            }
            Err(e) => {
                debug!("Error accepting connection: {}", e);
            }
        }
    }
}
