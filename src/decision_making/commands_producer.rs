use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use decision_making::decision::Decision;
use factions::Factions;

pub trait CommandsProducer {
    fn decide(
        &self,
        active_card: u8,
        current_eligible: Factions,
        map: &Map,
        track: &Track,
        available_forces: &AvailableForces,
    ) -> Result<Decision, String>;
}
