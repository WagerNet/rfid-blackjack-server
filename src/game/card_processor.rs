use crate::protocol::CardData;
use super::card_lookup::CardLookup;
use log::{info, debug};

pub struct CardProcessor {
    lookup: CardLookup,
}

impl CardProcessor {
    pub fn new() -> Self {
        info!("Creating card processor");
        Self {
            lookup: CardLookup::new(),
        }
    }
    
    pub async fn process_card(&self, data: CardData) {
        let card_value = self.lookup.get_card_value(data.card_id);
        
        debug!("Processing card: {} (ID: {}) for {} at table {} at timestamp {}", 
               card_value, data.card_id, data.player_type(), data.table_id, data.timestamp);
        
        self.update_game_state(data, &card_value).await;
    }
    
    async fn update_game_state(&self, data: CardData, value: &str) {
        debug!("Game state updated for table {} with card {}", data.table_id, value);
    }
}