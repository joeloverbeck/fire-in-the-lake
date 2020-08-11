use std::collections::HashMap;

use cards::card::Card;
use factions::Factions;

pub struct CardRegistry {
    cards: HashMap<u8, Card>,
}

impl Default for CardRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CardRegistry {
    pub fn new() -> CardRegistry {
        let mut card_registry = CardRegistry {
            cards: HashMap::new(),
        };

        card_registry.register_all_game_cards();

        card_registry
    }

    fn register_all_game_cards(&mut self) {
        // Handles registring every single card and its appropriate codified detail.
        let trucks = Card::new([Factions::NVA, Factions::VC, Factions::US, Factions::ARVN]);
        self.cards.insert(55, trucks);

        let green_berets = Card::new([Factions::ARVN, Factions::US, Factions::VC, Factions::NVA]);
        self.cards.insert(68, green_berets);

        // Card 107: "Burning Bonze"
        let burning_bonze = Card::new([Factions::VC, Factions::NVA, Factions::ARVN, Factions::US]);
        self.cards.insert(107, burning_bonze);
    }

    pub fn get_card(&self, card_number: u8) -> Result<&Card, String> {
        if card_number == 0 {
            panic!("Attempted to get the card details of a card with the number 0. Either the active or the preview card hadn't been set.");
        }

        let retrieved_card_option = self.cards.get(&card_number);

        match retrieved_card_option {
            Some(card) => Ok(card),
            None => panic!(
                "Didn't find any card with the passed number {}",
                card_number
            ),
        }
    }
}
