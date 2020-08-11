use cards::card_registry::CardRegistry;
use decision_making::choices::Choices;
use factions::Factions;
use game_flow::cards_involved::CardsInvolved;
use game_flow::factions_possibilities_handler::FactionsPossibilitiesHandler;

pub struct GameFlowHandler<'a> {
    card_registry: &'a CardRegistry,
    cards_involved: CardsInvolved,
    faction_possibilities_handler: FactionsPossibilitiesHandler,
}

impl<'a> GameFlowHandler<'a> {
    pub fn new(card_registry: &'a CardRegistry) -> GameFlowHandler<'a> {
        GameFlowHandler {
            card_registry,
            cards_involved: CardsInvolved::new(),
            faction_possibilities_handler: FactionsPossibilitiesHandler::new(),
        }
    }

    pub fn set_active_card(&mut self, new_active_card: u8) -> Result<[Factions; 4], String> {
        let faction_order = self
            .cards_involved
            .set_active_card(new_active_card, &self.card_registry)?;

        // First, pass the information to the sequence of play in order to populate
        // its mantained list of eligible factions.
        // However, if at this point there is any faction that was eligible,
        // that was due to another previous turn having left factions eligible
        // for the next one, so this shouldn't be done.
        if !self.faction_possibilities_handler.is_any_faction_elegible() {
            self.faction_possibilities_handler
                .populate_eligible_factions(faction_order);
        }

        // We have the correct active card object in there.
        self.faction_possibilities_handler
            .set_current_eligible(faction_order[0]);

        Ok(faction_order)
    }

    pub fn get_active_card(&self) -> u8 {
        if self.cards_involved.get_active_card() == 0 {
            panic!("The program attempted to recover the active card when none had been set!");
        }

        self.cards_involved.get_active_card()
    }

    pub fn set_preview_card(&mut self, new_preview_card: u8) {
        self.cards_involved.set_preview_card(new_preview_card);
    }

    pub fn has_turn_ended(&self) -> bool {
        self.faction_possibilities_handler.get_current_eligible() == Factions::None
    }

    fn delegate_handling_changes_to_game_flow(
        &mut self,
        faction: Factions,
        choice: Choices,
    ) -> Result<(), String> {
        self.faction_possibilities_handler
            .handle_changes_to_faction_possibilities(faction, choice)?;

        Ok(())
    }

    fn move_to_next_eligible(&mut self, faction_that_decided: Factions) -> Result<(), String> {
        let active_card_object_result = self
            .card_registry
            .get_card(self.cards_involved.get_active_card());

        match active_card_object_result {
            Err(active_card_object) => {
                return Err(format!(
                    "Could not retrieve the card details for card {:?}. That should not happen.",
                    active_card_object
                ))
            }
            Ok(active_card_object) => {
                self.faction_possibilities_handler
                    .move_to_next_eligible(faction_that_decided, active_card_object)?;
            }
        }

        Ok(())
    }

    pub fn report_choice(&mut self, faction: Factions, choice: Choices) -> Result<(), String> {
        // If whoever called this reported the choice of a faction who isn't the current eligible, then something is wrong with the program.
        if faction != self.faction_possibilities_handler.get_current_eligible() {
            return Err(format!("Attempted to report a choice for faction {:?}, but the current eligible faction was {:?}", faction, self.faction_possibilities_handler.get_current_eligible()));
        }

        // The sequence of play object should be responsible for handling the proper changes to the flow, at least in that regard.
        self.delegate_handling_changes_to_game_flow(faction, choice)?;

        // Regardless of the choice, a choice has been made, so we need to move to the next eligible.
        // Let's get the faction order for the active card
        self.move_to_next_eligible(faction)?;

        Ok(())
    }

    pub fn perform_end_of_turn(&mut self) {
        self.faction_possibilities_handler.perform_end_of_turn();

        // Gotta exchange the active card for the preview card.
        self.cards_involved.exchange_active_card_for_preview_card();

        // Remains asking for the new preview card.
        // TODO
    }

    pub fn is_faction_eligible(&self, faction: Factions) -> bool {
        self.faction_possibilities_handler
            .is_faction_eligible(faction)
    }

    pub fn number_of_eligible_factions(&self) -> usize {
        self.faction_possibilities_handler
            .number_of_eligible_factions()
    }

    pub fn are_all_factions_elegible(&self) -> bool {
        self.faction_possibilities_handler
            .are_all_factions_elegible()
    }

    pub fn is_execute_op_and_special_activity_available(&self) -> bool {
        self.faction_possibilities_handler
            .is_execute_op_and_special_activity_available()
    }

    pub fn get_current_elegible(&self) -> Factions {
        self.faction_possibilities_handler.get_current_eligible()
    }

    pub fn faction_present_in_second_op_and_special_activity(&self) -> Factions {
        self.faction_possibilities_handler
            .faction_present_in_second_op_and_special_activity()
    }

    pub fn get_current_eligible(&self) -> Factions {
        self.faction_possibilities_handler.get_current_eligible()
    }

    pub fn faction_present_in_second_limited_operation(&self) -> Factions {
        self.faction_possibilities_handler
            .faction_present_in_second_limited_operation()
    }

    pub fn has_faction_passed(&self, faction: Factions) -> bool {
        self.faction_possibilities_handler
            .has_faction_passed(faction)
    }

    pub fn faction_present_in_first_eligible_event(&self) -> Factions {
        self.faction_possibilities_handler
            .faction_present_in_first_eligible_event()
    }

    pub fn faction_present_in_operation_only(&self) -> Factions {
        self.faction_possibilities_handler
            .faction_present_in_operation_only()
    }

    pub fn set_faction_as_ineligible(&mut self, faction: Factions) -> Result<(), String> {
        self.faction_possibilities_handler
            .set_faction_as_ineligible(faction)?;

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

        let game_flow_handler = GameFlowHandler::new(&card_registry);

        Ok(())
    }

    #[test]
    fn test_should_be_able_to_set_and_retrieve_active_card() -> Result<(), String> {
        let card_registry = CardRegistry::new();

        let mut game_flow_handler = GameFlowHandler::new(&card_registry);

        game_flow_handler.set_active_card(107)?;

        assert_eq!(
            game_flow_handler.cards_involved.get_active_card(),
            107,
            "The active card should have been 107"
        );

        Ok(())
    }

    #[test]
    fn test_on_init_all_factions_should_be_eligible() {
        let card_registry = CardRegistry::new();
        let mut game_flow_handler = GameFlowHandler::new(&card_registry);

        assert_eq!(4, game_flow_handler.number_of_eligible_factions());
    }
}
