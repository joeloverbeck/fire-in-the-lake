use game_definitions::factions::Factions;
use sequence_of_play::domain::movements::Movements;
use sequence_of_play::domain::produce_sequence_of_play_movements::produce_sequence_of_play_movements;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;

pub struct SequenceOfPlayController {
    first_eligible: Option<Factions>,
    second_eligible: Option<Factions>,
    first_faction_operation_only: Option<Factions>,
    first_faction_event: Option<Factions>,
    passed: Vec<Factions>,
    ineligible: Vec<Factions>,
}

impl Default for SequenceOfPlayController {
    fn default() -> Self {
        Self::new()
    }
}

impl SequenceOfPlayController {
    pub fn new() -> SequenceOfPlayController {
        SequenceOfPlayController {
            first_eligible: None,
            second_eligible: None,
            first_faction_operation_only: None,
            first_faction_event: None,
            passed: Vec::new(),
            ineligible: Vec::new(),
        }
    }

    pub fn register_faction_order(&mut self, faction_order: [Factions; 4]) -> Result<(), String> {
        // Here is where we have in mind the factions that are ineligible.
        for faction in faction_order.iter() {
            if !self
                .ineligible
                .iter()
                .any(|ineligible_faction| faction == ineligible_faction)
                && self.first_eligible.is_none()
            {
                self.first_eligible = Some(*faction);
            } else if !self
                .ineligible
                .iter()
                .any(|ineligible_faction| faction == ineligible_faction)
                && self.second_eligible.is_none()
            {
                self.second_eligible = Some(*faction)
            }

            if self.first_eligible.is_some() && self.second_eligible.is_some() {
                break;
            }
        }

        Ok(())
    }

    pub fn get_first_eligible(&self) -> Result<Factions, String> {
        // Sanity check
        if self.first_eligible.is_none() {
            panic!("Attempted to get the first eligible faction from the sequence of play, but it was none. You should first ask whether there is any.");
        }

        Ok(*self.first_eligible.as_ref().unwrap())
    }

    pub fn get_second_eligible(&self) -> Result<Factions, String> {
        if self.second_eligible.is_none() {
            panic!("Attempted to get the second eligible faction from the sequence of play, but it was none. You should first ask whether there is any.");
        }

        Ok(*self.second_eligible.as_ref().unwrap())
    }

    pub fn get_current_elegible_faction(&self) -> &Option<Factions> {
        if self.first_eligible.is_none() && self.second_eligible.is_none() {
            panic!("Asked which was the current elegible faction, but none were! The asking code should have asked whether there were eligible factions first.");
        }

        if self.first_eligible.is_some() {
            &self.first_eligible
        } else {
            &self.second_eligible
        }
    }

    pub fn register_pick(
        &mut self,
        faction: &Factions,
        faction_order: [Factions; 4],
        slot: &SequenceOfPlaySlots,
    ) -> Result<(), String> {
        let movements = produce_sequence_of_play_movements(faction, faction_order, slot, &self)?;

        // Just go through all of them and persist them.
        for movement in movements {
            match *movement.get_movement() {
                Movements::FirstEligible => {
                    // Note that the "faction" present in the mutation might be None.
                    if movement.does_it_contain_a_faction() {
                        self.first_eligible = Some(*movement.get_faction());
                    } else {
                        self.first_eligible = None;
                    }
                }
                Movements::FirstFactionOperationOnly => {
                    // Note that the "faction" present in the mutation might be None.
                    if movement.does_it_contain_a_faction() {
                        self.first_faction_operation_only = Some(*movement.get_faction());
                    } else {
                        self.first_faction_operation_only = None;
                    }
                }
                Movements::FirstFactionEvent => {
                    // Note that the "faction" present in the mutation might be None.
                    if movement.does_it_contain_a_faction() {
                        self.first_faction_event = Some(*movement.get_faction());
                    } else {
                        self.first_faction_event = None;
                    }
                }
                Movements::Passed => self.passed.push(*movement.get_faction()),
                Movements::SecondEligible => {
                    // Note that the "faction" present in the mutation might be None.
                    if movement.does_it_contain_a_faction() {
                        self.second_eligible = Some(*movement.get_faction());
                    } else {
                        self.second_eligible = None;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn is_faction_ineligible(&self, faction: &Factions) -> Result<bool, String> {
        Ok(self
            .ineligible
            .iter()
            .any(|ineligible_faction| *ineligible_faction == *faction))
    }

    pub fn get_faction_in_first_faction_event(&self) -> Result<Factions, String> {
        Ok(*self.first_faction_event.as_ref().unwrap())
    }

    pub fn is_first_faction_event_taken(&self) -> Result<bool, String> {
        Ok(self.first_faction_event.is_some())
    }

    pub fn get_possible_actions_for_current_elegible(
        &self,
    ) -> Result<Vec<SequenceOfPlaySlots>, String> {
        if self.first_eligible.is_some() {
            let mut vec: Vec<SequenceOfPlaySlots> = Vec::new();
            vec.push(SequenceOfPlaySlots::FirstFactionOperationOnly);
            vec.push(SequenceOfPlaySlots::FirstFactionOperationPlusSpecialActivity);
            vec.push(SequenceOfPlaySlots::FirstFactionEvent);
            vec.push(SequenceOfPlaySlots::Pass);

            return Ok(vec);
        }

        if self.second_eligible.is_some() {
            // Obviously there's plenty to do here, depending on what the previous
            // eligible has done.

            if self.first_faction_operation_only.is_some() {
                // The first eligible chose to play a single operation, without special activities.

                // Sanity check
                if self.first_faction_event.is_some() {
                    panic!("While determining possible actions for current eligible, found that both first faction operation only and first faction event were filled!");
                }

                let mut vec: Vec<SequenceOfPlaySlots> = Vec::new();
                vec.push(SequenceOfPlaySlots::SecondFactionLimitedOperation);
                vec.push(SequenceOfPlaySlots::Pass);

                return Ok(vec);
            } else {
                panic!("Failed to implemented all possibilities for second eligible.");
            }
        }

        Err("Was asked to figure out the possible operations for the current eligible faction, but couldn't find any. Probably the asking code failed to ask first whether there was any faction eligible.".to_string())
    }

    pub fn is_there_a_first_eligible_faction(&self) -> Result<bool, String> {
        Ok(self.first_eligible.is_some())
    }

    pub fn is_there_a_second_eligible_faction(&self) -> Result<bool, String> {
        Ok(self.second_eligible.is_some())
    }

    pub fn is_there_a_next_eligible_faction(&self) -> Result<bool, String> {
        Ok(self.first_eligible.is_some() || self.second_eligible.is_some())
    }

    pub fn perform_end_of_turn(&mut self) -> Result<(), String> {
        // Sanity check: by the end of the turn, there should be no first nor second eligible factions.
        if self.first_eligible.is_some() || self.second_eligible.is_some() {
            panic!("While performing end of turn sequence, found out that either the first eligible or the second were some!");
        }

        // First, all factions in ineligible should "leave" that vec, so they are considered eligible for
        // next turn.
        self.ineligible.clear();

        // Those factions that have chosen anything different than passing should go to ineligible.
        if self.first_faction_event.is_some() {
            self.ineligible
                .push(self.first_faction_event.take().unwrap());
        }
        if self.first_faction_operation_only.is_some() {
            self.ineligible
                .push(self.first_faction_operation_only.take().unwrap());
        }

        // Also clear the passed vec.
        self.passed.clear();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_upon_setting_a_faction_order_with_no_previous_turns_played_the_first_and_second_eligible_factions_are_the_ones_expected(
    ) -> Result<(), String> {
        let mut sut = SequenceOfPlayController::new();

        sut.register_faction_order([Factions::US, Factions::ARVN, Factions::VC, Factions::NVA])?;

        let first_eligible = sut.get_first_eligible()?;
        let second_eligible = sut.get_second_eligible()?;

        assert_eq!(first_eligible, Factions::US);

        assert_eq!(second_eligible, Factions::ARVN);

        Ok(())
    }

    #[test]
    fn test_when_asking_for_the_current_elegible_faction_after_registering_the_faction_order_the_returned_current_elegible_is_the_expected_one(
    ) -> Result<(), String> {
        let mut sut = SequenceOfPlayController::new();

        sut.register_faction_order([Factions::US, Factions::ARVN, Factions::VC, Factions::NVA])?;

        let possible_current_elegible_faction = sut.get_current_elegible_faction();

        if possible_current_elegible_faction.is_none() {
            panic!("The current elegible faction should have been the US, but it was none!");
        }

        Ok(())
    }

    #[test]
    fn test_if_all_four_factions_pass_after_the_last_pass_there_are_no_eligible_factions(
    ) -> Result<(), String> {
        let mut sut = SequenceOfPlayController::new();

        let faction_order = [Factions::ARVN, Factions::VC, Factions::NVA, Factions::US];

        sut.register_faction_order(faction_order)?;

        sut.register_pick(&Factions::ARVN, faction_order, &SequenceOfPlaySlots::Pass)?;
        sut.register_pick(&Factions::VC, faction_order, &SequenceOfPlaySlots::Pass)?;
        sut.register_pick(&Factions::NVA, faction_order, &SequenceOfPlaySlots::Pass)?;
        sut.register_pick(&Factions::US, faction_order, &SequenceOfPlaySlots::Pass)?;

        assert_eq!(
            sut.is_there_a_first_eligible_faction()?,
            false,
            "After all factions having passed, there should be no first eligible faction."
        );
        assert_eq!(
            sut.is_there_a_second_eligible_faction()?,
            false,
            "After all factions having passed, there should be no second eligible faction."
        );

        Ok(())
    }

    #[test]
    fn test_after_performing_end_of_turn_when_all_factions_have_passed_the_next_turn_all_faction_should_be_eligible_in_faction_order(
    ) -> Result<(), String> {
        let mut sut = SequenceOfPlayController::new();

        let faction_order = [Factions::ARVN, Factions::VC, Factions::NVA, Factions::US];

        sut.register_faction_order(faction_order)?;

        sut.register_pick(&Factions::ARVN, faction_order, &SequenceOfPlaySlots::Pass)?;
        sut.register_pick(&Factions::VC, faction_order, &SequenceOfPlaySlots::Pass)?;
        sut.register_pick(&Factions::NVA, faction_order, &SequenceOfPlaySlots::Pass)?;
        sut.register_pick(&Factions::US, faction_order, &SequenceOfPlaySlots::Pass)?;

        sut.perform_end_of_turn()?;

        let new_faction_order = [Factions::VC, Factions::US, Factions::ARVN, Factions::NVA];

        sut.register_faction_order(new_faction_order)?;

        assert!(sut.is_there_a_first_eligible_faction()?);
        assert!(sut.is_there_a_second_eligible_faction()?);

        assert_eq!(sut.get_first_eligible()?, Factions::VC);
        assert_eq!(sut.get_second_eligible()?, Factions::US);

        Ok(())
    }

    #[test]
    fn test_after_performing_end_of_turn_when_a_faction_has_chosen_first_faction_operation_only_it_shouldnt_be_eligible_next_turn(
    ) -> Result<(), String> {
        let mut sut = SequenceOfPlayController::new();

        let faction_order = [Factions::ARVN, Factions::VC, Factions::NVA, Factions::US];

        sut.register_faction_order(faction_order)?;

        sut.register_pick(&Factions::ARVN, faction_order, &SequenceOfPlaySlots::Pass)?;
        sut.register_pick(
            &Factions::VC,
            faction_order,
            &SequenceOfPlaySlots::FirstFactionOperationOnly,
        )?;
        sut.register_pick(&Factions::NVA, faction_order, &SequenceOfPlaySlots::Pass)?;
        sut.register_pick(&Factions::US, faction_order, &SequenceOfPlaySlots::Pass)?;

        sut.perform_end_of_turn()?;

        let new_faction_order = [Factions::VC, Factions::US, Factions::ARVN, Factions::NVA];

        sut.register_faction_order(new_faction_order)?;

        assert!(sut.is_there_a_first_eligible_faction()?);
        assert!(sut.is_there_a_second_eligible_faction()?);

        assert_eq!(sut.get_first_eligible()?, Factions::US);
        assert_eq!(sut.get_second_eligible()?, Factions::ARVN);

        Ok(())
    }
}
