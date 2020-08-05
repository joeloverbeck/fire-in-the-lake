use factions::Factions;
use cards::card_registry::CardRegistry;


pub struct Card {
    number: u8,
    faction_order: [Factions; 4]
}

impl Card {
    pub fn new(number: u8, faction_order: [Factions; 4]) -> Card{
        Card {
            number: number,
            faction_order: faction_order
        }
    }

    fn retrieve_faction_order(&self) -> [Factions; 4] {
        self.faction_order
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_should_be_able_to_create_a_card() -> Result<(), String> {
         
        let _ = Card::new(107, [Factions::None; 4]);

        Ok(())
    }

    #[test]
    fn test_the_card_107_should_have_the_expected_faction_order() -> Result<(), String> {

        let card_registry = CardRegistry::new();

        let burning_bonze_card = card_registry.retrieve_card(107);

        match burning_bonze_card {
            Err(error) => panic!(error),
            Ok(card) => {
                let faction_order = card.retrieve_faction_order();

                assert_eq!(faction_order[0], Factions::VC);
                assert_eq!(faction_order[1], Factions::NVA);
                assert_eq!(faction_order[2], Factions::ARVN);
                assert_eq!(faction_order[3], Factions::US);
            }
        }

        Ok(())
    }

}