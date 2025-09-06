use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;
use log::{info, debug};

#[derive(Debug, Clone)]
pub struct TableConnection {
    pub table_id: u8,
    pub address: SocketAddr,
    pub connected_at: SystemTime,
}

impl TableConnection {
    pub fn new(table_id: u8, address: SocketAddr) -> Self {
        Self {
            table_id,
            address,
            connected_at: SystemTime::now(),
        }
    }
}

#[derive(Clone)]
pub struct ConnectionPool {
    connections: Arc<Mutex<HashMap<u8, TableConnection>>>,
}

impl ConnectionPool {
    pub fn new() -> Self {
        info!("Creating connection pool");
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn add_connection(&self, table_id: u8, address: SocketAddr) {
        let mut connections = self.connections.lock().await;
        let connection = TableConnection::new(table_id, address);
        connections.insert(table_id, connection);
        
        let total = connections.len();
        debug!("Added connection for table {} from {} (total active: {})", table_id, address, total);
    }
    
    pub async fn remove_connection(&self, table_id: u8) {
        let mut connections = self.connections.lock().await;
        if let Some(conn) = connections.remove(&table_id) {
            let total = connections.len();
            debug!("Removed connection for table {} from {} (total active: {})", 
                   table_id, conn.address, total);
        }
    }
    
    pub async fn get_connection(&self, table_id: u8) -> Option<TableConnection> {
        let connections = self.connections.lock().await;
        connections.get(&table_id).cloned()
    }
    
    pub async fn active_connections(&self) -> usize {
        let connections = self.connections.lock().await;
        connections.len()
    }
}