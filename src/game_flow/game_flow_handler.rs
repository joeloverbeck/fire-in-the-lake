

use cards::card_registry::CardRegistry;
use game_flow::sequence_of_play::SequenceOfPlay;

struct GameFlowHandler<'a> {
    card_registry: &'a CardRegistry,
    sequence_of_play: &'a SequenceOfPlay,
    active_card: u8
}

impl <'a>GameFlowHandler<'a> {
    fn new(card_registry: &'a CardRegistry, sequence_of_play: &'a SequenceOfPlay) -> GameFlowHandler<'a> {
        GameFlowHandler {
            card_registry: card_registry,
            sequence_of_play: sequence_of_play,
            active_card: 0
        }
    }

    fn get_active_card(&self) -> u8 {
        self.active_card
    }

    fn set_active_card(&mut self, new_active_card: u8){
        self.active_card = new_active_card
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_can_create_game_flow_handler() -> Result<(), String> {
        let card_registry = CardRegistry::new();
        let sequence_of_play = SequenceOfPlay::new();

        let game_flow_handler = GameFlowHandler::new(&card_registry, &sequence_of_play);

        Ok(())
    }


    #[test]
    fn test_should_be_able_to_set_and_retrieve_active_card() -> Result<(), String> {
        let card_registry = CardRegistry::new();
        let sequence_of_play = SequenceOfPlay::new();

        let mut game_flow_handler = GameFlowHandler::new(&card_registry, &sequence_of_play);

        game_flow_handler.set_active_card(107);

        assert_eq!(game_flow_handler.get_active_card(), 107, "The active card should have been 107");

        Ok(())
    }
}