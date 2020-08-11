use cards::card::Card;
use cards::card_registry::CardRegistry;
use decision_making::choices::Choices;
use factions::get_position_in_faction_order_of_faction::get_position_in_faction_order_of_faction;
use factions::Factions;

pub struct GameFlowHandler<'a> {
    card_registry: &'a CardRegistry,
    eligible: [Factions; 4],
    ineligible: [Factions; 4],
    passed: [Factions; 4],
    first_eligible_event: Factions,
    operation_only: Factions,
    second_limited_operation: Factions,
    second_op_and_special_activity: Factions,
    active_card: u8,
    preview_card: u8,
    current_eligible: Factions,
}

impl<'a> GameFlowHandler<'a> {
    pub fn new(card_registry: &'a CardRegistry) -> GameFlowHandler<'a> {
        GameFlowHandler {
            card_registry,
            eligible: [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US],
            ineligible: [
                Factions::None,
                Factions::None,
                Factions::None,
                Factions::None,
            ],
            passed: [
                Factions::None,
                Factions::None,
                Factions::None,
                Factions::None,
            ],
            first_eligible_event: Factions::None,
            operation_only: Factions::None,
            second_limited_operation: Factions::None,
            second_op_and_special_activity: Factions::None,
            active_card: 0,
            preview_card: 0,
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
                // However, if at this point there is any faction that was eligible,
                // that was due to another previous turn having left factions eligible
                // for the next one, so this shouldn't be done.
                if !self.is_any_faction_elegible() {
                    self.populate_eligible_factions(active_card_object.get_faction_order());
                }

                // We have the correct active card object in there.
                self.current_eligible = active_card_object.get_faction_order()[0];
            }
        }

        Ok(())
    }

    pub fn get_preview_card(&self) -> u8 {
        self.preview_card
    }

    pub fn set_preview_card(&mut self, card_number: u8) {
        self.preview_card = card_number;
    }

    pub fn has_turn_ended(&self) -> bool {
        self.get_current_eligible() == Factions::None
    }

    pub fn set_faction_as_ineligible(&mut self, faction: Factions) -> Result<(), String> {
        self.move_faction_to_ineligible(faction)?;

        Ok(())
    }

    pub fn get_current_eligible(&self) -> Factions {
        // The current eligible should be the faction that currently could decide what to do.
        // Note: it could be that there is no current elegible, and thus the turn has ended.
        if !self.is_any_faction_elegible() {
            return Factions::None;
        }

        self.current_eligible
    }

    pub fn is_execute_op_and_special_activity_available(&self) -> bool {
        // As a norm, execute Op and Special Activity should be available if there is a faction in "first_eligible_event"
        // TODO: consider whether this is asked by the first eligible.
        self.faction_present_in_first_eligible_event() != Factions::None
            && self.faction_present_in_second_op_and_special_activity() == Factions::None
    }

    fn delegate_moving_faction_to_first_eligible_event(
        &mut self,
        faction: Factions,
    ) -> Result<(), String> {
        if let Err(error) = self.move_faction_to_first_eligible_event(faction) {
            panic!(format!(
                "Couldn't move faction {:?} to first eligible event! Error: {:?}",
                faction, error
            ));
        }

        Ok(())
    }

    fn delegate_handling_changes_to_game_flow(
        &mut self,
        faction: Factions,
        choice: Choices,
    ) -> Result<(), String> {
        match choice {
            Choices::SecondLimitedOperation => {
                if let Err(error) = self.move_faction_to_second_limited_operation(faction) {
                    panic!(format!(
                        "Couldn't move faction {:?} to second limited operation! Error: {:?}",
                        faction, error
                    ));
                }
            }
            Choices::OperationOnly => {
                if let Err(error) = self.move_faction_to_operation_only(faction) {
                    panic!(format!(
                        "Couldn't move faction {:?} to operation only! Error: {:?}",
                        faction, error
                    ));
                }
            }
            Choices::UnshadedEvent => {
                self.delegate_moving_faction_to_first_eligible_event(faction)?;
            }
            Choices::ShadedEvent => {
                self.delegate_moving_faction_to_first_eligible_event(faction)?;
            }
            Choices::Pass => {
                // Must move the appropriate faction to the passed array, in a position in which it won't step
                // on any other that might have passed.
                if let Err(error) = self.move_faction_to_pass(faction) {
                    panic!("Attempted to move the faction {:?} to the passed box, but couldn't! Error: {:?}", faction, error);
                }
            }
            Choices::SecondOperationAndSpecialActivity => {
                // Must move the appropriate faction to the slot that identifies having chosen an Operation with Special Activity
                if let Err(error) =
                    self.move_faction_to_second_operation_and_special_activity(faction)
                {
                    panic!("Attempted to move the faction {:?} to the operation and special activity box, but couldn't! Error: {:?}", faction, error);
                }
            }
        }

        Ok(())
    }

    fn determine_next_actual_eligible_faction_from_position_of_last_eligible_faction(
        &mut self,
        card: &Card,
        position_of_last_eligible_faction: usize,
    ) -> Result<(), String> {
        let mut index = position_of_last_eligible_faction;

        // Note: the next one in line might very well be ineligible.
        // Careful not to go out of bounds.
        while index < 3 && self.is_faction_ineligible(card.get_faction_order()[index + 1]) {
            index += 1;
        }

        // Again, careful with the bounds.
        if (index + 1) < 4 {
            self.current_eligible = card.get_faction_order()[index + 1];
        }

        Ok(())
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
                // Obviously the next current elegible should be the faction to the right in the faction order,
                // UNLESS that faction is uneligible.
                let possible_position_of_last_eligible_faction =
                    get_position_in_faction_order_of_faction(
                        active_card_object,
                        faction_that_decided,
                    );

                if let Some(position_of_last_eligible_faction) =
                    possible_position_of_last_eligible_faction
                {
                    self.determine_next_actual_eligible_faction_from_position_of_last_eligible_faction(active_card_object, position_of_last_eligible_faction.to_owned())?;
                } else {
                    return Err(format!("Had attempted to locate the position of the faction {:?} in the faction order of the current card: {:?}, but couldn't do it!", faction_that_decided, active_card_object.get_faction_order()));
                }
            }
        }

        Ok(())
    }

    pub fn report_choice(&mut self, faction: Factions, choice: Choices) -> Result<(), String> {
        // If whoever called this reported the choice of a faction who isn't the current eligible, then something is wrong with the program.
        if faction != self.current_eligible {
            return Err(format!("Attempted to report a choice for faction {:?}, but the current eligible faction was {:?}", faction, self.current_eligible));
        }

        // The sequence of play object should be responsible for handling the proper changes to the flow, at least in that regard.
        self.delegate_handling_changes_to_game_flow(faction, choice)?;

        // Regardless of the choice, a choice has been made, so we need to move to the next eligible.
        // Let's get the faction order for the active card
        self.move_to_next_eligible(faction)?;

        Ok(())
    }

    pub fn number_of_eligible_factions(&self) -> usize {
        // The number of eligible factions is obviously those in elegible that aren't Faction::None
        let mut amount_of_elegible = 0;

        for faction in self.eligible.iter() {
            match faction {
                Factions::ARVN => amount_of_elegible += 1,
                Factions::NVA => amount_of_elegible += 1,
                Factions::VC => amount_of_elegible += 1,
                Factions::US => amount_of_elegible += 1,
                _ => (),
            }
        }

        amount_of_elegible
    }

    pub fn has_faction_passed(&self, faction: Factions) -> bool {
        self.passed
            .iter()
            .any(|eligible_faction| eligible_faction == &faction)
    }

    pub fn are_all_factions_elegible(&self) -> bool {
        // All factions would be elegible when every one of the four factions (not NONE) are elegible.
        let mut vc_present: bool = false;
        let mut nva_present: bool = false;
        let mut arvn_present: bool = false;
        let mut us_present: bool = false;

        for faction in self.eligible.iter() {
            match faction {
                Factions::ARVN => arvn_present = true,
                Factions::NVA => nva_present = true,
                Factions::VC => vc_present = true,
                Factions::US => us_present = true,
                _ => (),
            }
        }

        vc_present && nva_present && arvn_present && us_present
    }

    fn remove_faction_to_move_from_eligibles(
        &mut self,
        faction_to_move: Factions,
    ) -> Result<(), String> {
        // First we gotta make sure it's there. Nobody should call this function if the faction to remove isn't
        // in the elegibles to begin with.
        if !self
            .eligible
            .iter()
            .any(|eligible_faction| eligible_faction == &faction_to_move)
        {
            return Err(format!("Was ordered to remove the faction {:?} from the list of elegibles, but it wasn't there in the first place. The list is: {:?}", faction_to_move, self.eligible));
        }

        for item in self.eligible.iter_mut() {
            match *item {
                _ if item == &faction_to_move => *item = Factions::None,
                _ => (),
            }
        }

        Ok(())
    }

    pub fn populate_eligible_factions(&mut self, faction_order: [Factions; 4]) {
        self.eligible[0] = faction_order[0];
        self.eligible[1] = faction_order[1];
        self.eligible[2] = faction_order[2];
        self.eligible[3] = faction_order[3];
    }

    fn move_faction_to_eligible(&mut self, faction_to_move: Factions) {
        for item in self.eligible.iter_mut() {
            match *item {
                _ if item == &Factions::None => {
                    *item = faction_to_move;
                    break;
                }
                _ => (),
            }
        }
    }

    fn check_eligibility_and_remove_faction_to_move(
        &mut self,
        faction_to_move: Factions,
    ) -> Result<(), String> {
        // Sanity check: the passed faction should be in the list of eligible factions.
        self.faction_should_be_movable_sanity_check(faction_to_move)?;
        self.remove_faction_to_move_from_eligibles(faction_to_move)?;

        Ok(())
    }

    pub fn move_faction_to_ineligible(&mut self, faction_to_move: Factions) -> Result<(), String> {
        self.check_eligibility_and_remove_faction_to_move(faction_to_move)?;

        self.slot_faction_in_ineligible(faction_to_move);

        Ok(())
    }

    pub fn move_faction_to_first_eligible_event(
        &mut self,
        faction_to_move: Factions,
    ) -> Result<(), String> {
        self.check_eligibility_and_remove_faction_to_move(faction_to_move)?;

        // With the faction to move eliminated from the Eligible, now we should put it in the box asked for.
        self.first_eligible_event = faction_to_move;

        Ok(())
    }

    pub fn move_faction_to_second_operation_and_special_activity(
        &mut self,
        faction_to_move: Factions,
    ) -> Result<(), String> {
        self.check_eligibility_and_remove_faction_to_move(faction_to_move)?;

        // If there is a faction already set where I'm going to put this one, it's a mistake in the code somewhere.
        if self.second_op_and_special_activity != Factions::None {
            return Err(format!("I was going to move the faction {:?} to the 'second operation and special activity', but the faction {:?} was already there!", faction_to_move, self.second_op_and_special_activity));
        }

        // Coast is clear
        self.second_op_and_special_activity = faction_to_move;

        Ok(())
    }

    pub fn move_faction_to_second_limited_operation(
        &mut self,
        faction_to_move: Factions,
    ) -> Result<(), String> {
        self.check_eligibility_and_remove_faction_to_move(faction_to_move)?;

        // If there is a faction already set where I'm going to put this one, it's a mistake in the code somewhere.
        if self.second_limited_operation != Factions::None {
            return Err(format!("I was going to move the faction {:?} to the 'second limited operation', but the faction {:?} was already there!", faction_to_move, self.second_limited_operation));
        }

        // Coast is clear
        self.second_limited_operation = faction_to_move;

        Ok(())
    }

    pub fn move_faction_to_pass(&mut self, faction_to_move: Factions) -> Result<(), String> {
        self.check_eligibility_and_remove_faction_to_move(faction_to_move)?;

        for item in self.passed.iter_mut() {
            match *item {
                _ if item == &Factions::None => {
                    *item = faction_to_move;
                    // Exit the loop lest I fill the passed array with this faction.
                    break;
                }
                _ => (),
            }
        }

        Ok(())
    }

    pub fn move_faction_to_operation_only(
        &mut self,
        faction_to_move: Factions,
    ) -> Result<(), String> {
        self.check_eligibility_and_remove_faction_to_move(faction_to_move)?;

        // If there is a faction already set where I'm going to put this one, it's a mistake in the code somewhere.
        if self.operation_only != Factions::None {
            return Err(format!("I was going to move the faction {:?} to the 'operation only' box, but the faction {:?} was already there!", faction_to_move, self.operation_only));
        }

        // Coast is clear
        self.operation_only = faction_to_move;

        Ok(())
    }

    fn faction_should_be_movable_sanity_check(
        &self,
        faction_to_move: Factions,
    ) -> Result<(), String> {
        if !self.is_faction_eligible(faction_to_move) {
            return Err(format!(
                "The faction to move {:?} wasn't eligible to begin with.",
                faction_to_move
            ));
        }

        Ok(())
    }

    fn slot_faction_in_ineligible(&mut self, faction_to_slot: Factions) {
        for item in self.ineligible.iter_mut() {
            match *item {
                _ if item == &Factions::None => {
                    *item = faction_to_slot;
                    // Exit the loop lest I fill the ineligible array with this faction.
                    break;
                }
                _ => (),
            }
        }

        // Check exit contract: the passed faction should be in ineligible
        if !self.is_faction_ineligible(faction_to_slot) {
            panic!(
                "Called to move {:?} to ineligibles, but after code ran, the faction wasn't there!",
                faction_to_slot
            );
        }
    }

    pub fn perform_end_of_turn(&mut self) {
        // Should move those ineligible to eligible.
        for index in 0..4 {
            if self.ineligible[index] != Factions::None {
                self.move_faction_to_eligible(self.ineligible[index]);
                self.ineligible[index] = Factions::None;
            }
        }

        // Should move those in passed to eligible.
        for index in 0..4 {
            if self.passed[index] != Factions::None {
                self.move_faction_to_eligible(self.passed[index]);
                self.passed[index] = Factions::None;
            }
        }

        // The factions that were in boxes different than eligible or pass should go to ineligible.
        if self.first_eligible_event != Factions::None {
            // Move that faction to ineligible.
            self.slot_faction_in_ineligible(self.first_eligible_event);
            self.first_eligible_event = Factions::None;
        }

        if self.second_op_and_special_activity != Factions::None {
            // Move that faction to ineligible.
            self.slot_faction_in_ineligible(self.second_op_and_special_activity);
            self.second_op_and_special_activity = Factions::None;
        }

        // Should move those in operation only to ineligible
        if self.operation_only != Factions::None {
            self.slot_faction_in_ineligible(self.operation_only);
            self.operation_only = Factions::None;
        }

        // Should move those in second limited operation to ineligible
        if self.second_limited_operation != Factions::None {
            self.slot_faction_in_ineligible(self.second_limited_operation);
            self.second_limited_operation = Factions::None;
        }

        // Gotta exchange the active card for the preview card.
        self.active_card = self.preview_card;

        // Remains asking for the new preview card.
        // TODO
    }

    pub fn is_any_faction_elegible(&self) -> bool {
        // Could be a few cases.
        // 1. There are literally no eligible factions in eligibles.
        if !self
            .eligible
            .iter()
            .any(|eligible_faction| eligible_faction != &Factions::None)
        {
            return false;
        }

        // Box per box: if there is one in the box corresponding to the first player,
        // and then there is another in the following box, then there's no faction elegible
        if self.first_eligible_event != Factions::None
            && self.second_op_and_special_activity != Factions::None
        {
            return false;
        }

        true
    }

    pub fn is_faction_eligible(&self, faction: Factions) -> bool {
        self.eligible
            .iter()
            .any(|eligible_faction| eligible_faction == &faction)
    }

    pub fn is_faction_ineligible(&self, faction: Factions) -> bool {
        self.ineligible
            .iter()
            .any(|ineligible_faction| ineligible_faction == &faction)
    }

    pub fn faction_present_in_first_eligible_event(&self) -> Factions {
        self.first_eligible_event
    }

    pub fn faction_present_in_operation_only(&self) -> Factions {
        self.operation_only
    }

    pub fn faction_present_in_second_op_and_special_activity(&self) -> Factions {
        self.second_op_and_special_activity
    }

    pub fn faction_present_in_second_limited_operation(&self) -> Factions {
        self.second_limited_operation
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

        game_flow_handler.set_active_card(107);

        assert_eq!(
            game_flow_handler.get_active_card(),
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

    #[test]
    fn test_on_init_all_elegible_factions_should_be_none() {
        let card_registry = CardRegistry::new();
        let mut game_flow_handler = GameFlowHandler::new(&card_registry);

        assert_eq!(game_flow_handler.are_all_factions_elegible(), true);
    }

    #[test]
    fn test_after_moving_an_eligible_faction_to_first_faction_event_it_should_be_in_that_box_and_missing_from_eligible(
    ) -> Result<(), String> {
        let card_registry = CardRegistry::new();
        let mut game_flow_handler = GameFlowHandler::new(&card_registry);

        if let Err(error) = game_flow_handler.move_faction_to_first_eligible_event(Factions::VC) {
            panic!(error);
        }

        assert_eq!(
            game_flow_handler.faction_present_in_first_eligible_event(),
            Factions::VC,
            "The VC faction wasn't in the first faction event box"
        );
        assert_eq!(game_flow_handler.is_faction_eligible(Factions::VC), false);
        Ok(())
    }

    #[test]
    fn test_after_moving_a_different_eligible_faction_to_first_faction_event_it_should_be_in_that_box_and_missing_from_eligible(
    ) -> Result<(), String> {
        let card_registry = CardRegistry::new();
        let mut game_flow_handler = GameFlowHandler::new(&card_registry);

        if let Err(error) = game_flow_handler.move_faction_to_first_eligible_event(Factions::US) {
            panic!(error);
        }

        assert_eq!(
            game_flow_handler.faction_present_in_first_eligible_event(),
            Factions::US,
            "The US faction wasn't in the first faction event box"
        );
        assert_eq!(game_flow_handler.is_faction_eligible(Factions::US), false);
        Ok(())
    }
}
