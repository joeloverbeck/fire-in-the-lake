
use factions::Factions;
use decision_making::decision::Decision;

pub trait CommandsProducer {
    fn decide(&self, active_card: u8, current_eligible: Factions) -> Decision;
}