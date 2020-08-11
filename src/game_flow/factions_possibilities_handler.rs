use cards::card::Card;
use decision_making::choices::Choices;
use factions::get_position_in_faction_order_of_faction::get_position_in_faction_order_of_faction;
use factions::Factions;

pub struct FactionsPossibilitiesHandler {
    current_eligible: Factions,
    eligible: [Factions; 4],
    ineligible: [Factions; 4],
    passed: [Factions; 4],
    first_eligible_event: Factions,
    operation_only: Factions,
    second_limited_operation: Factions,
    second_op_and_special_activity: Factions,
}

impl Default for FactionsPossibilitiesHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl FactionsPossibilitiesHandler {
    pub fn new() -> FactionsPossibilitiesHandler {
        FactionsPossibilitiesHandler {
            current_eligible: Factions::None,
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
        }
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

    pub fn is_faction_eligible(&self, faction: Factions) -> bool {
        self.eligible
            .iter()
            .any(|eligible_faction| eligible_faction == &faction)
    }

    pub fn is_execute_op_and_special_activity_available(&self) -> bool {
        // As a norm, execute Op and Special Activity should be available if there is a faction in "first_eligible_event"
        // TODO: consider whether this is asked by the first eligible.
        self.faction_present_in_first_eligible_event() != Factions::None
            && self.faction_present_in_second_op_and_special_activity() == Factions::None
    }

    pub fn move_to_next_eligible(
        &mut self,
        faction_that_decided: Factions,
        card: &Card,
    ) -> Result<(), String> {
        // We have the correct active card object in there.
        // Obviously the next current elegible should be the faction to the right in the faction order,
        // UNLESS that faction is uneligible.
        let possible_position_of_last_eligible_faction =
            get_position_in_faction_order_of_faction(card, faction_that_decided);

        if let Some(position_of_last_eligible_faction) = possible_position_of_last_eligible_faction
        {
            self.determine_next_actual_eligible_faction_from_position_of_last_eligible_faction(
                card,
                position_of_last_eligible_faction.to_owned(),
            )?;
        } else {
            return Err(format!("Had attempted to locate the position of the faction {:?} in the faction order of the current card: {:?}, but couldn't do it!", faction_that_decided, card.get_faction_order()));
        }

        Ok(())
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

    pub fn populate_eligible_factions(&mut self, faction_order: [Factions; 4]) {
        self.eligible[0] = faction_order[0];
        self.eligible[1] = faction_order[1];
        self.eligible[2] = faction_order[2];
        self.eligible[3] = faction_order[3];
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

    pub fn is_faction_ineligible(&self, faction: Factions) -> bool {
        self.ineligible
            .iter()
            .any(|ineligible_faction| ineligible_faction == &faction)
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

    pub fn set_current_eligible(&mut self, faction: Factions) {
        self.current_eligible = faction;
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

    pub fn handle_changes_to_faction_possibilities(
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
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_on_init_all_elegible_factions_should_be_none() {
        let handler = FactionsPossibilitiesHandler::new();

        assert_eq!(handler.are_all_factions_elegible(), true);
    }

    #[test]
    fn test_after_moving_an_eligible_faction_to_first_faction_event_it_should_be_in_that_box_and_missing_from_eligible(
    ) -> Result<(), String> {
        let mut handler = FactionsPossibilitiesHandler::new();

        if let Err(error) = handler.move_faction_to_first_eligible_event(Factions::VC) {
            panic!(error);
        }

        assert_eq!(
            handler.faction_present_in_first_eligible_event(),
            Factions::VC,
            "The VC faction wasn't in the first faction event box"
        );
        assert_eq!(handler.is_faction_eligible(Factions::VC), false);
        Ok(())
    }

    #[test]
    fn test_after_moving_a_different_eligible_faction_to_first_faction_event_it_should_be_in_that_box_and_missing_from_eligible(
    ) -> Result<(), String> {
        let mut handler = FactionsPossibilitiesHandler::new();

        if let Err(error) = handler.move_faction_to_first_eligible_event(Factions::US) {
            panic!(error);
        }

        assert_eq!(
            handler.faction_present_in_first_eligible_event(),
            Factions::US,
            "The US faction wasn't in the first faction event box"
        );
        assert_eq!(handler.is_faction_eligible(Factions::US), false);
        Ok(())
    }
}
