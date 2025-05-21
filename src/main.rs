mod config;
mod models;
mod protocol;
mod server;

use config::Config;
use std::{env, io};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yml".to_string());
    let config = Config::load(config_path).expect("Failed to load config");

    let listener = TcpListener::bind("0.0.0.0:25565").await?;
    println!("Server listening on port 25565");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("New connection from: {}", addr);

                let config = config.clone();
                tokio::spawn(async move {
                    if let Err(e) = server::handle_client(stream, config).await {
                        eprintln!("Error handling client {}: {}", addr, e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
