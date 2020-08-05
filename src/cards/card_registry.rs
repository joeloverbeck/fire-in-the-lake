
use std::collections::HashMap;

use cards::card::Card;
use factions::Factions;

pub struct CardRegistry {
    cards: HashMap<u8, Card>
}

impl CardRegistry {

    pub fn new() -> CardRegistry {
        let mut card_registry = CardRegistry {
            cards: HashMap::new()
        };

        card_registry.register_all_game_cards();

        card_registry
    }

    fn register_all_game_cards(&mut self) {
        // Handles registring every single card and its appropriate codified detail.
        // Card 107: "Burning Bonze"
        let burning_bonze = Card::new(107, [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US]);

        self.insert_game_card(107, burning_bonze);
    }

    fn insert_game_card(&mut self, number: u8, card_to_insert: Card) {
        self.cards.insert(number, card_to_insert);
    }

    pub fn retrieve_card(&self, card_number: u8) -> Result<&Card, String> {
        let retrieved_card_option = self.cards.get(&card_number);

        match retrieved_card_option {
            Some(card) => { 
                return Ok(card)
            },
            None => panic!("Didn't find any card with the passed number {}", card_number)
        }
    }
}