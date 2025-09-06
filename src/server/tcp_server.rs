use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use std::sync::Arc;
use log::{info, warn, error, debug};

use crate::config::ServerConfig;
use crate::protocol::{CardData, CARD_DATA_SIZE, ACK_RESPONSE};
use crate::game::CardProcessor;
use super::connection_pool::ConnectionPool;

pub struct TcpServerManager {
    config: ServerConfig,
    connection_pool: ConnectionPool,
    processor: Arc<CardProcessor>,
}

impl TcpServerManager {
    pub fn new(config: ServerConfig) -> Self {
        info!("Creating TCP server manager");
        Self {
            config,
            connection_pool: ConnectionPool::new(),
            processor: Arc::new(CardProcessor::new()),
        }
    }
    
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting TCP server on {} tables", self.config.max_tables);
        
        for (table_id, &port) in self.config.ports.iter().enumerate() {
            if table_id >= self.config.max_tables as usize {
                break;
            }
            
            let table_num = table_id as u8 + 1;
            info!("Starting server for table {} on port {}", table_num, port);
            
            let config = self.config.clone();
            let pool = self.connection_pool.clone();
            let processor = Arc::clone(&self.processor);
            
            tokio::spawn(async move {
                match Self::run_table_server(table_num, port, config, pool, processor).await {
                    Ok(_) => info!("Table {} server stopped", table_num),
                    Err(e) => error!("Table {} server error: {}", table_num, e),
                }
            });
        }
        
        info!("All table servers started");
        Ok(())
    }
    
    async fn run_table_server(
        table_id: u8,
        port: u16,
        config: ServerConfig,
        pool: ConnectionPool,
        processor: Arc<CardProcessor>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bind_addr = format!("{}:{}", config.host, port);
        info!("Table {} binding to {}", table_id, bind_addr);
        
        let listener = match TcpListener::bind(&bind_addr).await {
            Ok(listener) => {
                info!("Table {} listening on {}", table_id, bind_addr);
                listener
            },
            Err(e) => {
                error!("Failed to bind table {} to {}: {}", table_id, bind_addr, e);
                return Err(e.into());
            }
        };
        
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    info!("Table {} accepted connection from {}", table_id, addr);
                    pool.add_connection(table_id, addr).await;
                    
                    let pool_clone = pool.clone();
                    let processor_clone = Arc::clone(&processor);
                    
                    tokio::spawn(async move {
                        Self::handle_connection(stream, table_id, addr, pool_clone, processor_clone).await;
                    });
                },
                Err(e) => {
                    error!("Table {} failed to accept connection: {}", table_id, e);
                }
            }
        }
    }
    
    async fn handle_connection(
        mut stream: TcpStream,
        table_id: u8,
        addr: SocketAddr,
        pool: ConnectionPool,
        processor: Arc<CardProcessor>,
    ) {
        info!("Table {} handling connection from {}", table_id, addr);
        let mut buffer = [0u8; CARD_DATA_SIZE];
        let mut card_count = 0;
        
        loop {
            match stream.read_exact(&mut buffer).await {
                Ok(_) => {
                    debug!("Table {} received {} bytes from {}", table_id, CARD_DATA_SIZE, addr);
                    
                    match CardData::from_bytes(&buffer) {
                        Ok(card_data) => {
                            card_count += 1;
                            debug!("Table {} processed card {} from {} (total: {})", 
                                   table_id, card_data.card_id, addr, card_count);
                            
                            processor.process_card(card_data).await;
                            
                            if let Err(e) = stream.write_all(&[ACK_RESPONSE]).await {
                                warn!("Table {} failed to send ACK to {}: {}", table_id, addr, e);
                                break;
                            }
                        },
                        Err(e) => {
                            warn!("Table {} invalid card data from {}: {}", table_id, addr, e);
                        }
                    }
                }
                Err(e) => {
                    info!("Table {} connection from {} closed: {}", table_id, addr, e);
                    break;
                }
            }
        }
        
        info!("Table {} disconnected {} (processed {} cards)", table_id, addr, card_count);
        pool.remove_connection(table_id).await;
    }
}