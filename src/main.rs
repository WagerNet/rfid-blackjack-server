mod config;
mod protocol;
mod server;
mod game;

use config::ServerConfig;
use server::TcpServerManager;
use log::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    info!("Starting card server");
    
    let config = ServerConfig::default();
    info!("Server config loaded - Max tables: {}, Host: {}", config.max_tables, config.host);
    
    let mut server = TcpServerManager::new(config);
    
    match server.start().await {
        Ok(_) => info!("Server started successfully"),
        Err(e) => {
            error!("Failed to start server: {}", e);
            return Err(e);
        }
    }
    
    info!("Server running, press Ctrl+C to stop");
    tokio::signal::ctrl_c().await?;
    info!("Server shutdown initiated");
    
    Ok(())
}