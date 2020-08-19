use cards::domain::card::Card;
use game_definitions::factions::Factions;

#[derive(Debug, Clone)]
pub struct CoupCard {
    number: u8,
    name: String,
}

impl Card for CoupCard {
    fn get_number(&self) -> Result<u8, String> {
        Ok(self.number)
    }

    fn get_name(&self) -> Result<String, String> {
        Ok((self.name).to_string())
    }

    fn get_faction_order(&self) -> Result<[Factions; 4], String> {
        panic!("Asked a coup card for its faction order. They never have any.");
    }

    fn has_any_faction_capability(&self) -> Result<bool, String> {
        Ok(false)
    }

    fn get_faction_capability(&self) -> Result<Factions, String> {
        panic!("Asked a coup card for its action capability. They never have any.");
    }

    fn is_coup_card(&self) -> Result<bool, String> {
        Ok(true)
    }

    fn is_pivotal_event_card(&self) -> Result<bool, String> {
        Ok(false)
    }
}

impl CoupCard {
    pub fn new(number: u8, name: String) -> CoupCard {
        CoupCard { number, name }
    }
}
