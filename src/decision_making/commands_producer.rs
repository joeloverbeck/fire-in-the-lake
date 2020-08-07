use decision_making::decision::Decision;
use factions::Factions;

pub trait CommandsProducer {
    fn decide(&self, active_card: u8, current_eligible: Factions) -> Decision;
}
