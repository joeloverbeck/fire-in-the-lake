use game_definitions::factions::Factions;
use sequence_of_play::domain::movements::Movements;

#[derive(Debug)]
pub struct MovementMutation {
    faction: Factions,
    movement: Movements,
}

impl MovementMutation {
    pub fn new(faction: Factions, movement: Movements) -> MovementMutation {
        MovementMutation { faction, movement }
    }

    pub fn get_faction(&self) -> &Factions {
        &self.faction
    }

    pub fn get_movement(&self) -> &Movements {
        &self.movement
    }
}
