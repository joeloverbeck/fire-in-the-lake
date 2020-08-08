use cards::card_registry::CardRegistry;
use decision_making::choices::Choices;
use factions::Factions;
use game_flow::sequence_of_play::SequenceOfPlay;

pub struct GameFlowHandler<'a> {
    card_registry: &'a CardRegistry,
    sequence_of_play: &'a mut SequenceOfPlay,
    active_card: u8,
    current_eligible: Factions,
}

impl<'a> GameFlowHandler<'a> {
    pub fn new(
        card_registry: &'a CardRegistry,
        sequence_of_play: &'a mut SequenceOfPlay,
    ) -> GameFlowHandler<'a> {
        GameFlowHandler {
            card_registry,
            sequence_of_play,
            active_card: 0,
            current_eligible: Factions::None,
        }
    }

    pub fn get_active_card(&self) -> u8 {
        self.active_card
    }

    pub fn set_active_card(&mut self, new_active_card: u8) -> Result<(), String> {
        self.active_card = new_active_card;

        // When setting the active card, the whole sequence of play should get reset.
        // We start by putting the current eligible as the first in the corresponding
        // faction order of the card
        let active_card_object_result = self.card_registry.get_card(self.active_card);

        match active_card_object_result {
            Err(active_card_object) => {
                return Err(format!(
                    "Could not retrieve the card details for card {:?}. That should not happen.",
                    active_card_object
                ))
            }
            Ok(active_card_object) => {
                // First, pass the information to the sequence of play in order to populate
                // its mantained list of eligible factions.
                self.sequence_of_play
                    .populate_eligible_factions(active_card_object.get_faction_order());

                // We have the correct active card object in there.
                self.current_eligible = active_card_object.get_faction_order()[0];
            }
        }

        Ok(())
    }

    pub fn get_current_eligible(&self) -> Factions {
        // The current eligible should be the faction that currently could decide what to do.
        // Note: it could be that there is no current elegible, and thus the turn has ended.
        if !self.sequence_of_play.is_any_faction_elegible() {
            return Factions::None;
        }

        self.current_eligible
    }

    pub fn is_faction_eligible(&self, faction: Factions) -> bool {
        self.sequence_of_play.is_faction_eligible(faction)
    }

    pub fn faction_present_in_first_eligible_event(&self) -> Factions {
        self.sequence_of_play
            .faction_present_in_first_eligible_event()
    }

    pub fn faction_present_in_op_and_special_activity(&self) -> Factions {
        self.sequence_of_play
            .faction_present_in_op_and_special_activity()
    }

    pub fn is_execute_op_and_special_activity_available(&self) -> bool {
        // As a norm, execute Op and Special Activity should be available if there is a faction in "first_eligible_event"
        // TODO: consider whether this is asked by the first eligible.
        self.faction_present_in_first_eligible_event() != Factions::None
            && self.faction_present_in_op_and_special_activity() == Factions::None
    }

    fn delegate_handling_changes_to_game_flow(&mut self, faction: Factions, choice: Choices) {
        match choice {
            Choices::ShadedEvent => {
                if let Err(error) = self
                    .sequence_of_play
                    .move_faction_to_first_eligible_event(faction)
                {
                    panic!(format!(
                        "Couldn't move faction {:?} to first eligible event! Error: {:?}",
                        faction, error
                    ));
                }
            }
            Choices::Pass => {
                // Must move the appropriate faction to the passed array, in a position in which it won't step
                // on any other that might have passed.
                if let Err(error) = self.sequence_of_play.move_faction_to_pass(faction) {
                    panic!("Attempted to move the faction {:?} to the passed box, but couldn't! Error: {:?}", faction, error);
                }
            }
            Choices::Operation => {
                // Must move the appropriate faction to the slot that identifies having chosen an Operation with Special Activity
                if let Err(error) = self
                    .sequence_of_play
                    .move_faction_to_operation_and_special_activity(faction)
                {
                    panic!("Attempted to move the faction {:?} to the operation and special activity box, but couldn't! Error: {:?}", faction, error);
                }
            }
        }
    }

    fn move_to_next_eligible(&mut self, faction_that_decided: Factions) -> Result<(), String> {
        let active_card_object_result = self.card_registry.get_card(self.active_card);

        match active_card_object_result {
            Err(active_card_object) => {
                return Err(format!(
                    "Could not retrieve the card details for card {:?}. That should not happen.",
                    active_card_object
                ))
            }
            Ok(active_card_object) => {
                // We have the correct active card object in there.
                // Obviously the next current elegible should be the faction to the right in the faction order.
                let possible_position_of_last_eligible_faction = active_card_object
                    .get_faction_order()
                    .iter()
                    .enumerate()
                    .filter_map(|faction_in_order| {
                        if faction_in_order.1 == &faction_that_decided {
                            Some(faction_in_order.0)
                        } else {
                            None
                        }
                    })
                    .next();

                if let Some(position_of_last_eligible_faction) =
                    possible_position_of_last_eligible_faction
                {
                    self.current_eligible = active_card_object.get_faction_order()
                        [position_of_last_eligible_faction + 1];
                } else {
                    return Err(format!("Had attempted to locate the position of the faction {:?} in the faction order of the current card: {:?}, but couldn't do it!", faction_that_decided, active_card_object.get_faction_order()));
                }
            }
        }

        Ok(())
    }

    pub fn has_faction_passed(&self, faction: Factions) -> bool {
        self.sequence_of_play.has_faction_passed(faction)
    }

    pub fn report_choice(&mut self, faction: Factions, choice: Choices) -> Result<(), String> {
        // If whoever called this reported the choice of a faction who isn't the current eligible, then something is wrong with the program.
        if faction != self.current_eligible {
            return Err(format!("Attempted to report a choice for faction {:?}, but the current eligible faction was {:?}", faction, self.current_eligible));
        }

        // The sequence of play object should be responsible for handling the proper changes to the flow, at least in that regard.
        self.delegate_handling_changes_to_game_flow(faction, choice);

        // Regardless of the choice, a choice has been made, so we need to move to the next eligible.
        // Let's get the faction order for the active card
        self.move_to_next_eligible(faction)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_can_create_game_flow_handler() -> Result<(), String> {
        let card_registry = CardRegistry::new();
        let mut sequence_of_play = SequenceOfPlay::new();

        let game_flow_handler = GameFlowHandler::new(&card_registry, &mut sequence_of_play);

        Ok(())
    }

    #[test]
    fn test_should_be_able_to_set_and_retrieve_active_card() -> Result<(), String> {
        let card_registry = CardRegistry::new();
        let mut sequence_of_play = SequenceOfPlay::new();

        let mut game_flow_handler = GameFlowHandler::new(&card_registry, &mut sequence_of_play);

        game_flow_handler.set_active_card(107);

        assert_eq!(
            game_flow_handler.get_active_card(),
            107,
            "The active card should have been 107"
        );

        Ok(())
    }
}
