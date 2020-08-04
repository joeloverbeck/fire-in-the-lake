

struct NvaStats {
    victory_level: u8
}

impl NvaStats{
    fn new () -> NvaStats {
        NvaStats {
            victory_level: 0
        }
    }

    fn get_victory_level(&self) -> u8 {
        self.victory_level
    }
}

struct Tracker {
    aid: u8
}

impl Tracker {
    fn new () -> Tracker {
        Tracker {
            aid: 0
        }
    }

    fn get_aid(&self) -> u8 {
        self.aid
    }
}

pub fn main() {
 println!("This sorta works.");

 
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Factions {
    NONE,
    VC,
    NVA,
    ARVN,
    US
}

struct SequenceOfPlay {
    eligible: [Factions; 4]
}

impl SequenceOfPlay {
    fn new() -> SequenceOfPlay {
        SequenceOfPlay {
            eligible: [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US]
        }
    }

    fn number_of_eligible_factions(&self) -> usize {
        self.eligible.len()
    }

    fn are_all_factions_elegible(&self) -> bool {
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
    fn test_on_creation_nva_victory_level_should_be_zero() {

        let nva_stats = NvaStats::new();

        assert_eq!(nva_stats.get_victory_level(), 0);
    }

    #[test]
    fn test_on_creation_aid_should_be_zero(){
        let tracker = Tracker::new();

        assert_eq!(tracker.get_aid(), 0);
    }


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