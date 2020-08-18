use game_definitions::factions::Factions;
use sequence_of_play::domain::movements::Movements;

#[derive(Debug)]
pub struct MovementMutation {
    possible_faction: Option<Factions>,
    movement: Movements,
}

impl MovementMutation {
    pub fn new(possible_faction: Option<Factions>, movement: Movements) -> MovementMutation {
        MovementMutation {
            possible_faction,
            movement,
        }
    }

    pub fn does_it_contain_a_faction(&self) -> bool {
        self.possible_faction.is_some()
    }

    pub fn get_faction(&self) -> &Factions {
        &self.possible_faction.as_ref().unwrap()
    }

    pub fn get_movement(&self) -> &Movements {
        &self.movement
    }
}
