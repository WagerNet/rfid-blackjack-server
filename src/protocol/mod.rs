use log::warn;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CardData {
    pub table_id: u8,
    pub card_id: u32,
    pub timestamp: u32,
    pub player: u8,
}

impl CardData {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != 10 {
            warn!("Invalid card data length: {} bytes (expected 10)", bytes.len());
            return Err("Invalid data length");
        }
        
        let mut data = CardData {
            table_id: 0,
            card_id: 0,
            timestamp: 0,
            player: 0,
        };
        
        data.table_id = bytes[0];
        data.card_id = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        data.timestamp = u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]);
        data.player = bytes[9];
        
        Ok(data)
    }
    
    pub fn player_type(&self) -> &str {
        match self.player {
            0 => "Dealer",
            _ => "Player",
        }
    }
}

pub const CARD_DATA_SIZE: usize = 10;
pub const ACK_RESPONSE: u8 = 0x01;