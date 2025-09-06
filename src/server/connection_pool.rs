use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;

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
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn add_connection(&self, table_id: u8, address: SocketAddr) {
        let mut connections = self.connections.lock().await;
        let connection = TableConnection::new(table_id, address);
        connections.insert(table_id, connection);
    }
    
    pub async fn remove_connection(&self, table_id: u8) {
        let mut connections = self.connections.lock().await;
        connections.remove(&table_id);
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