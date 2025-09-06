use std::collections::HashMap;

pub struct CardLookup {
    card_map: HashMap<u32, String>,
}

impl CardLookup {
    pub fn new() -> Self {
        let mut lookup = Self {
            card_map: HashMap::new(),
        };
        lookup.init_cards();
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
        self.card_map.get(&card_id)
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string())
    }
    
    pub fn add_card(&mut self, card_id: u32, value: String) {
        self.card_map.insert(card_id, value);
    }
}