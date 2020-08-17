use game_definitions::factions::Factions;
use sequence_of_play::domain::movements::Movements;
use sequence_of_play::domain::produce_sequence_of_play_movements::produce_sequence_of_play_movements;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;

pub struct SequenceOfPlayController {
    first_eligible: Option<Factions>,
    second_eligible: Option<Factions>,
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
            first_faction_event: None,
            passed: Vec::new(),
            ineligible: Vec::new(),
        }
    }

    pub fn register_faction_order(&mut self, faction_order: [Factions; 4]) -> Result<(), String> {
        self.first_eligible = Some(faction_order[0]);
        self.second_eligible = Some(faction_order[1]);

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

        &self.first_eligible
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
                Movements::FirstEligible => self.first_eligible = Some(*movement.get_faction()),
                Movements::FirstFactionEvent => {
                    self.first_faction_event = Some(*movement.get_faction())
                }
                Movements::Passed => self.passed.push(*movement.get_faction()),
                Movements::SecondEligible => self.second_eligible = Some(*movement.get_faction()),
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

    pub fn get_possible_actions_for_current_elegible(&self) -> Result<Vec<String>, String> {
        if self.first_eligible.is_some() {
            let mut vec: Vec<String> = Vec::new();
            vec.push("operation".to_string());
            vec.push("pass".to_string());
            vec.push("event".to_string());

            return Ok(vec);
        }

        Err("Was asked to figure out the possible operations for the current eligible faction, but couldn't find any. Probably the asking code failed to ask first whether there was any faction elegible.".to_string())
    }

    pub fn is_there_a_next_eligible_faction(&self) -> Result<bool, String> {
        Ok(self.first_eligible.is_some() || self.second_eligible.is_some())
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
    fn test_when_asking_for_a_string_with_possible_actions_for_current_elegible_it_returns_expected_ones(
    ) -> Result<(), String> {
        let mut sut = SequenceOfPlayController::new();

        sut.register_faction_order([Factions::US, Factions::ARVN, Factions::VC, Factions::NVA])?;

        let possible_operations_for_current_elegible =
            sut.get_possible_actions_for_current_elegible()?;

        assert_eq!(possible_operations_for_current_elegible[0], "operation");
        assert_eq!(possible_operations_for_current_elegible[1], "pass");
        assert_eq!(possible_operations_for_current_elegible[2], "event");

        Ok(())
    }
}
