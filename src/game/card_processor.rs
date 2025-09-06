use crate::protocol::CardData;
use super::card_lookup::CardLookup;

pub struct CardProcessor {
    lookup: CardLookup,
}

impl CardProcessor {
    pub fn new() -> Self {
        Self {
            lookup: CardLookup::new(),
        }
    }
    
    pub async fn process_card(&self, data: CardData) {
        let card_value = self.lookup.get_card_value(data.card_id);
        
        self.update_game_state(data, &card_value).await;
    }
    
    async fn update_game_state(&self, _data: CardData, _value: &str) {
    }
}