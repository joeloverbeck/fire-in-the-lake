extern crate enum_dispatch;
use self::enum_dispatch::enum_dispatch;
use cards::domain::coup_card::CoupCard;
use cards::domain::pivotal_event_card::PivotalEventCard;
use cards::domain::regular_card::RegularCard;
use game_definitions::factions::Factions;

#[enum_dispatch]
#[allow(clippy::too_many_arguments)]
pub trait Card {
    fn get_number(&self) -> Result<u8, String>;
    fn get_name(&self) -> Result<String, String>;
    fn get_faction_order(&self) -> Result<[Factions; 4], String>;
    fn has_any_faction_capability(&self) -> Result<bool, String>;
    fn get_faction_capability(&self) -> Result<Factions, String>;
    fn is_coup_card(&self) -> Result<bool, String>;
    fn is_pivotal_event_card(&self) -> Result<bool, String>;
}

#[enum_dispatch(Card)]
#[derive(Debug, Clone)]
pub enum Cards {
    RegularCard,
    CoupCard,
    PivotalEventCard,
}
