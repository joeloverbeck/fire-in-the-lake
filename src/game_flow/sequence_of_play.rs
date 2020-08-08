use factions::Factions;

pub struct SequenceOfPlay {
    eligible: [Factions; 4],
    passed: [Factions; 4],
    first_eligible_event: Factions,
}

impl Default for SequenceOfPlay {
    fn default() -> Self {
        Self::new()
    }
}

impl SequenceOfPlay {
    pub fn new() -> SequenceOfPlay {
        SequenceOfPlay {
            eligible: [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US],
            passed: [
                Factions::None,
                Factions::None,
                Factions::None,
                Factions::None,
            ],
            first_eligible_event: Factions::None,
        }
    }

    pub fn number_of_eligible_factions(&self) -> usize {
        self.eligible.len()
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

    fn remove_faction_to_move_from_eligibles(&mut self, faction_to_move: Factions) {
        for item in self.eligible.iter_mut() {
            match *item {
                _ if item == &faction_to_move => *item = Factions::None,
                _ => (),
            }
        }
    }

    pub fn move_faction_to_pass(&mut self, faction_to_move: Factions) -> Result<(), String> {
        self.faction_should_be_movable_sanity_check(faction_to_move)?;

        self.remove_faction_to_move_from_eligibles(faction_to_move);

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

    pub fn move_faction_to_first_eligible_event(
        &mut self,
        faction_to_move: Factions,
    ) -> Result<(), String> {
        // Sanity check: the passed faction should be in the list of eligible factions.
        self.faction_should_be_movable_sanity_check(faction_to_move)?;

        self.remove_faction_to_move_from_eligibles(faction_to_move);

        // With the faction to move eliminated from the Eligible, now we should put it in the box asked for.
        self.first_eligible_event = faction_to_move;

        Ok(())
    }

    pub fn is_faction_eligible(&self, faction: Factions) -> bool {
        self.eligible
            .iter()
            .any(|eligible_faction| eligible_faction == &faction)
    }

    pub fn faction_present_in_first_eligible_event(&self) -> Factions {
        self.first_eligible_event
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_on_init_all_factions_should_be_eligible() {
        let sequence_of_play = SequenceOfPlay::new();

        assert_eq!(4, sequence_of_play.number_of_eligible_factions());
    }

    #[test]
    fn test_on_init_all_elegible_factions_should_be_none() {
        let sequence_of_play = SequenceOfPlay::new();

        assert_eq!(sequence_of_play.are_all_factions_elegible(), true);
    }

    #[test]
    fn test_after_moving_an_eligible_faction_to_first_faction_event_it_should_be_in_that_box_and_missing_from_eligible(
    ) -> Result<(), String> {
        let mut sequence_of_play = SequenceOfPlay::new();

        if let Err(error) = sequence_of_play.move_faction_to_first_eligible_event(Factions::VC) {
            panic!(error);
        }

        assert_eq!(
            sequence_of_play.faction_present_in_first_eligible_event(),
            Factions::VC,
            "The VC faction wasn't in the first faction event box"
        );
        assert_eq!(sequence_of_play.is_faction_eligible(Factions::VC), false);
        Ok(())
    }

    #[test]
    fn test_after_moving_a_different_eligible_faction_to_first_faction_event_it_should_be_in_that_box_and_missing_from_eligible(
    ) -> Result<(), String> {
        let mut sequence_of_play = SequenceOfPlay::new();

        if let Err(error) = sequence_of_play.move_faction_to_first_eligible_event(Factions::US) {
            panic!(error);
        }

        assert_eq!(
            sequence_of_play.faction_present_in_first_eligible_event(),
            Factions::US,
            "The US faction wasn't in the first faction event box"
        );
        assert_eq!(sequence_of_play.is_faction_eligible(Factions::US), false);
        Ok(())
    }
}
