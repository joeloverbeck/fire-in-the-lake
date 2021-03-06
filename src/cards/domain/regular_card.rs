use cards::domain::card::Card;
use game_definitions::factions::Factions;

#[derive(Debug, Clone)]
pub struct RegularCard {
    number: u8,
    name: String,
    faction_order: [Factions; 4],
    faction_capability: Option<Factions>,
}

impl Card for RegularCard {
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
        Ok(self.faction_capability.is_some())
    }

    fn get_faction_capability(&self) -> Result<Factions, String> {
        Ok(*self.faction_capability.as_ref().unwrap())
    }

    fn is_coup_card(&self) -> Result<bool, String> {
        Ok(false)
    }

    fn is_pivotal_event_card(&self) -> Result<bool, String> {
        Ok(false)
    }
}

impl RegularCard {
    pub fn new(
        number: u8,
        name: String,
        faction_order: [Factions; 4],
        faction_capability: Option<Factions>,
    ) -> RegularCard {
        RegularCard {
            number,
            name,
            faction_order,
            faction_capability,
        }
    }
}
