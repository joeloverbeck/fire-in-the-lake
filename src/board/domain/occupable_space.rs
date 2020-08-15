use board::domain::initialize_hashmap_of_forces::initialize_hashmap_of_forces;
use game_definitions::forces::Forces;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OccupableSpace {
    forces: HashMap<Forces, u8>,
}

impl Default for OccupableSpace {
    fn default() -> Self {
        Self::new()
    }
}

impl OccupableSpace {
    pub fn new() -> OccupableSpace {
        OccupableSpace {
            forces: initialize_hashmap_of_forces(),
        }
    }

    pub fn set_forces(&mut self, forces: Forces, value: u8) -> Result<(), String> {
        *self.forces.get_mut(&forces).unwrap() = value;

        Ok(())
    }

    pub fn get_forces(&self, forces: Forces) -> Result<u8, String> {
        Ok(*self.forces.get(&forces).unwrap())
    }
}
