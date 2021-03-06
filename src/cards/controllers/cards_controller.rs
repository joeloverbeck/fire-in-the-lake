use cards::domain::card::Card;
use cards::domain::card::Cards;
use cards::domain::create_collection_of_cards::create_collection_of_cards;
use game_definitions::factions::Factions;
use std::collections::HashMap;

pub struct CardsController {
    cards: HashMap<u8, Cards>,
    active_card_number: u8,
    preview_card_number: u8,
}

impl Default for CardsController {
    fn default() -> Self {
        Self::new()
    }
}

impl CardsController {
    pub fn new() -> CardsController {
        CardsController {
            cards: create_collection_of_cards(),
            active_card_number: 0,
            preview_card_number: 0,
        }
    }

    pub fn set_active_card(&mut self, number: u8) -> Result<(), String> {
        self.active_card_number = number;

        Ok(())
    }

    pub fn get_active_card(&self) -> Result<&Cards, String> {
        Ok(self.cards.get(&self.active_card_number).as_ref().unwrap())
    }

    pub fn get_active_card_number(&self) -> Result<u8, String> {
        Ok(self.active_card_number)
    }

    pub fn set_preview_card(&mut self, number: u8) -> Result<(), String> {
        self.preview_card_number = number;

        Ok(())
    }

    pub fn get_preview_card(&self) -> Result<&Cards, String> {
        Ok(self.cards.get(&self.preview_card_number).as_ref().unwrap())
    }

    pub fn get_active_card_name(&self) -> Result<String, String> {
        let card = self.cards.get(&self.active_card_number).unwrap();

        Ok(card.get_name()?)
    }

    pub fn get_faction_order(&self, number: u8) -> Result<[Factions; 4], String> {
        // Sanity check
        if !self.cards.contains_key(&number) {
            return Err(format!("Attempted to get the faction order for the card with the number {:?}, but couldn't find it in the registry of cards! Contents: {:?}", number, self.cards));
        }

        let card = self.cards.get(&number).unwrap();

        Ok(card.get_faction_order()?)
    }

    pub fn move_preview_card_to_active(&mut self) -> Result<(), String> {
        // Sanity check
        if self.preview_card_number == 0 {
            panic!("Attempted to move the preview card to active, but the preview card hadn't been set!");
        }

        self.active_card_number = self.preview_card_number;

        self.preview_card_number = 0;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_after_setting_any_active_card_you_can_return_their_faction_order() -> Result<(), String>
    {
        let mut sut = CardsController::new();

        for index in 1..124 {
            sut.set_preview_card(index)?;

            let faction_order = sut.get_faction_order(index)?;

            assert_eq!(faction_order.len(), 4);
        }

        Ok(())
    }
}
