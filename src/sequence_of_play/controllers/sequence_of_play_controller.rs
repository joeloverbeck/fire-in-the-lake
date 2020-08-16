use game_definitions::factions::Factions;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;

pub struct SequenceOfPlayController {
    first_eligible: Option<Factions>,
    second_eligible: Option<Factions>,
    first_faction_event: Option<Factions>,
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
        }
    }

    pub fn register_faction_order(&mut self, faction_order: [Factions; 4]) -> Result<(), String> {
        self.first_eligible = Some(faction_order[0]);
        self.second_eligible = Some(faction_order[1]);

        Ok(())
    }

    pub fn get_first_eligible(&self) -> &Option<Factions> {
        &self.first_eligible
    }

    pub fn get_second_eligible(&self) -> &Option<Factions> {
        &self.second_eligible
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
        slot: &SequenceOfPlaySlots,
    ) -> Result<(), String> {
        if slot == &SequenceOfPlaySlots::FirstFactionEvent {
            // The faction was the first elegible, and played for the event.
            if self.first_faction_event.is_some() {
                panic!("Had attempted to register {:?} as having chosen to play the event being the first elegible faction, but there was a faction already in that position!: {:?}", faction, self.first_faction_event.as_ref().unwrap());
            }

            self.first_faction_event = Some(*faction);
        } else {
            panic!("Registering a pick in the sequence of play wasn't handled for the following: {:?} and {:?}", faction, slot);
        }

        Ok(())
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

        let possible_first_eligible = sut.get_first_eligible();
        let possible_second_eligible = sut.get_second_eligible();

        assert!(
            !possible_first_eligible.is_none(),
            "There should have been a first eligible faction."
        );
        assert!(
            !possible_second_eligible.is_none(),
            "There should have been a second eligible faction."
        );

        if let Some(first_eligible) = possible_first_eligible {
            assert_eq!(first_eligible, &Factions::US);
        }

        if let Some(second_eligible) = possible_second_eligible {
            assert_eq!(second_eligible, &Factions::ARVN);
        }

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
