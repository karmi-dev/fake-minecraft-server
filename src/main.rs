mod config;
mod models;
mod protocol;
mod server;
mod shutdown;

use config::Config;
use shutdown::Shutdown;
use std::{env, io};
use tokio::{net::TcpListener, select};
use tracing::{debug, info, info_span, Instrument};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> io::Result<()> {
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yml".to_string());
    let mut config = match Config::load(config_path) {
        Ok(config) => config,
        Err(e) => {
            panic!("Failed to load config: {} ({:?})", e, e.kind());
        }
    };

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(
                // Set log level to "debug" if enabled in config or environment; default is "info"
                if !config.debug && env::var("DEBUG").map(|v| v != "true").unwrap_or(true) {
                    "info"
                } else {
                    "debug"
                },
            )
        }))
        .with_target(false)
        .init();

    config.handle_logs();
    config.handle_favicon();

    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;
    info!("Server listening on port {}", config.port);

    let mut shutdown = Shutdown::new()?;

    loop {
        select! {
            conn = listener.accept() => {
                match conn {
                    Ok((stream, addr)) => {
                        debug!("New connection from: {}", addr);

                        let config = config.clone();
                        tokio::spawn(
                            async move {
                                if let Err(e) = server::handle_client(stream, config).await {
                                    debug!("Error handling client: {}", e);
                                }
                            }
                            .instrument(info_span!("client", "{}", addr)),
                        );
                    }
                    Err(e) => {
                        debug!("Error accepting connection: {}", e);
                    }
                }
            }

            _ = shutdown.wait_for_shutdown() => {
                break;
            }
        }
    }

    info!("Shutting down server...");
    Ok(())
}
