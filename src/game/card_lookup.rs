use std::collections::HashMap;
use log::{info, warn};

pub struct CardLookup {
    card_map: HashMap<u32, String>,
}

impl CardLookup {
    pub fn new() -> Self {
        info!("Initializing card lookup table");
        let mut lookup = Self {
            card_map: HashMap::new(),
        };
        lookup.init_cards();
        info!("Card lookup initialized with {} cards", lookup.card_map.len());
        lookup
    }
    
    fn init_cards(&mut self) {
        let suits = ["S", "H", "D", "C"];
        let values = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K"];
        
        for (i, value) in values.iter().enumerate() {
            for (j, suit) in suits.iter().enumerate() {
                let card_id = (j * 13 + i + 1) as u32;
                let card_name = format!("{}{}", value, suit);
                self.card_map.insert(card_id, card_name);
            }
        }
    }
    
    pub fn get_card_value(&self, card_id: u32) -> String {
        match self.card_map.get(&card_id) {
            Some(value) => value.clone(),
            None => {
                warn!("Unknown card ID: {}", card_id);
                "Unknown".to_string()
            }
        }
    }
    
    pub fn add_card(&mut self, card_id: u32, value: String) {
        info!("Adding custom card: {} -> {}", card_id, value);
        self.card_map.insert(card_id, value);
    }
}