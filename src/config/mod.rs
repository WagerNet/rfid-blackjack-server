#[derive(Clone)]  
pub struct ServerConfig {
    pub ports: Vec<u16>,
    pub max_tables: u8,
    pub host: String,
}

impl ServerConfig {
    pub fn new() -> Self {
        Self {
            ports: vec![8888, 8889, 8890, 8891, 8892],
            max_tables: 5,
            host: "0.0.0.0".to_string(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self::new()
    }
}