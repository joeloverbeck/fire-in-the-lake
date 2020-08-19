use cards::domain::card::Card;
use game_definitions::factions::Factions;

#[derive(Debug, Clone)]
pub struct PivotalEventCard {
    number: u8,
    name: String,
    faction_order: [Factions; 4],
}

impl Card for PivotalEventCard {
    fn get_number(&self) -> Result<u8, String> {
        Ok(self.number)
    }

    fn get_name(&self) -> Result<String, String> {
        Ok((self.name).to_string())
    }

    fn get_faction_order(&self) -> Result<[Factions; 4], String> {
        Ok(self.faction_order)
    }

    fn has_any_faction_capability(&self) -> Result<bool, String> {
        Ok(false)
    }

    fn get_faction_capability(&self) -> Result<Factions, String> {
        panic!("Asked a pivotal event for its action capability. They never have any.");
    }

    fn is_coup_card(&self) -> Result<bool, String> {
        Ok(false)
    }

    fn is_pivotal_event_card(&self) -> Result<bool, String> {
        Ok(true)
    }
}

impl PivotalEventCard {
    pub fn new(number: u8, name: String, faction_order: [Factions; 4]) -> PivotalEventCard {
        PivotalEventCard {
            number,
            name,
            faction_order,
        }
    }
}
