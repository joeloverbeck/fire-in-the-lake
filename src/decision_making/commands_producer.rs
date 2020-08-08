use board::map::Map;
use board::track::Track;
use decision_making::decision::Decision;
use factions::Factions;

pub trait CommandsProducer<'a> {
    fn decide(
        &self,
        active_card: u8,
        current_eligible: Factions,
        map: &'a mut Map,
        track: &'a mut Track,
    ) -> Decision<'a>;
}
