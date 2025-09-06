mod config;
mod protocol;
mod server;
mod game;

use config::ServerConfig;
use server::TcpServerManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::default();
    let mut server = TcpServerManager::new(config);
    
    server.start().await?;
    
    tokio::signal::ctrl_c().await?;
    Ok(())
}