use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use std::sync::Arc;

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
        Self {
            config,
            connection_pool: ConnectionPool::new(),
            processor: Arc::new(CardProcessor::new()),
        }
    }
    
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for (table_id, &port) in self.config.ports.iter().enumerate() {
            if table_id >= self.config.max_tables as usize {
                break;
            }
            
            let config = self.config.clone();
            let pool = self.connection_pool.clone();
            let processor = Arc::clone(&self.processor);
            
            tokio::spawn(async move {
                let _ = Self::run_table_server(
                    table_id as u8 + 1, 
                    port, 
                    config, 
                    pool, 
                    processor
                ).await;
            });
        }
        
        Ok(())
    }
    
    async fn run_table_server(
        table_id: u8,
        port: u16,
        config: ServerConfig,
        pool: ConnectionPool,
        processor: Arc<CardProcessor>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("{}:{}", config.host, port)).await?;
        
        loop {
            let (stream, addr) = listener.accept().await?;
            pool.add_connection(table_id, addr).await;
            
            let pool_clone = pool.clone();
            let processor_clone = Arc::clone(&processor);
            
            tokio::spawn(async move {
                Self::handle_connection(stream, table_id, addr, pool_clone, processor_clone).await;
            });
        }
    }
    
    async fn handle_connection(
        mut stream: TcpStream,
        table_id: u8,
        addr: SocketAddr,
        pool: ConnectionPool,
        processor: Arc<CardProcessor>,
    ) {
        let mut buffer = [0u8; CARD_DATA_SIZE];
        
        loop {
            match stream.read_exact(&mut buffer).await {
                Ok(_) => {
                    if let Ok(card_data) = CardData::from_bytes(&buffer) {
                        processor.process_card(card_data).await;
                        let _ = stream.write_all(&[ACK_RESPONSE]).await;
                    }
                }
                Err(_) => break,
            }
        }
        
        pool.remove_connection(table_id).await;
    }
}