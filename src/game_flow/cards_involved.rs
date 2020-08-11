use cards::card_registry::CardRegistry;
use factions::Factions;

pub struct CardsInvolved {
    active_card: u8,
    preview_card: u8,
}

impl Default for CardsInvolved {
    fn default() -> Self {
        Self::new()
    }
}

impl CardsInvolved {
    pub fn new() -> CardsInvolved {
        CardsInvolved {
            active_card: 0,
            preview_card: 0,
        }
    }

    pub fn get_active_card(&self) -> u8 {
        self.active_card
    }

    pub fn set_active_card(
        &mut self,
        new_active_card: u8,
        card_registry: &CardRegistry,
    ) -> Result<[Factions; 4], String> {
        self.active_card = new_active_card;

        // When setting the active card, the whole sequence of play should get reset.
        // We start by putting the current eligible as the first in the corresponding
        // faction order of the card
        let active_card_object_result = card_registry.get_card(self.active_card);

        match active_card_object_result {
            Err(active_card_object) => Err(format!(
                "Could not retrieve the card details for card {:?}. That should not happen.",
                active_card_object
            )),
            Ok(active_card_object) => Ok(active_card_object.get_faction_order()),
        }
    }

    pub fn get_preview_card(&self) -> u8 {
        self.preview_card
    }

    pub fn set_preview_card(&mut self, card_number: u8) {
        self.preview_card = card_number;
    }

    pub fn exchange_active_card_for_preview_card(&mut self) {
        self.active_card = self.preview_card;
    }
}
