use super::Factions;

pub struct SequenceOfPlay {
    eligible: [Factions; 4]
}

impl SequenceOfPlay {
    pub fn new() -> SequenceOfPlay {
        SequenceOfPlay {
            eligible: [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US]
        }
    }

    pub fn number_of_eligible_factions(&self) -> usize {
        self.eligible.len()
    }

    pub fn are_all_factions_elegible(&self) -> bool {
        // All factions would be elegible when every one of the four factions (not NONE) are elegible.
        let mut vc_present:bool = false;
        let mut nva_present:bool = false;
        let mut arvn_present: bool = false;
        let mut us_present:bool = false;

        for faction in self.eligible.iter(){
            match faction {
                Factions::ARVN => arvn_present = true,
                Factions::NVA => nva_present = true,
                Factions::VC => vc_present = true,
                Factions::US => us_present = true,
                _ => ()
            }
        }

        vc_present && nva_present && arvn_present && us_present
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
    fn test_on_init_all_elegible_factions_should_be_none(){
        let sequence_of_play = SequenceOfPlay::new();

        assert_eq!(sequence_of_play.are_all_factions_elegible(), true);
    }
}