use board::domain::initialize_hashmap_of_forces::initialize_hashmap_of_forces;
use board::domain::space::Space;
use game_definitions::control_types::ControlTypes;
use game_definitions::forces::Forces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::support_levels::SupportLevels;
use game_definitions::terrain_types::TerrainTypes;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LoC {
    space_identifier: SpaceIdentifiers,
    forces: HashMap<Forces, u8>,
    geographic_area: GeographicAreas,
    adjacent_spaces: Vec<SpaceIdentifiers>,
    sabotage: bool,
}

impl Space for LoC {
    fn get_space_identifier(&self) -> Result<SpaceIdentifiers, String> {
        Ok(self.space_identifier)
    }

    fn set_forces(&mut self, forces: Forces, value: u8) -> Result<(), String> {
        *self.forces.get_mut(&forces).unwrap() = value;

        Ok(())
    }

    fn get_forces(&self, forces: Forces) -> Result<u8, String> {
        Ok(*self.forces.get(&forces).unwrap())
    }

    fn get_support_level(&self) -> Result<&SupportLevels, String> {
        panic!("Asked a LoC's support level. LoCs don't have support levels. Something is wrong with the calling code.");
    }

    fn set_support_level(&mut self, _support_level: SupportLevels) -> Result<(), String> {
        panic!("Attempted to set the support level of a LoC. LoCs don't have support levels. Something is wrong with the calling code.");
    }

    fn reduce_forces(&mut self, forces: &Forces, number: u8) -> Result<(), String> {
        *self.forces.get_mut(&forces).unwrap() -= number;

        Ok(())
    }

    fn increase_forces(&mut self, forces: &Forces, number: u8) -> Result<(), String> {
        *self.forces.get_mut(&forces).unwrap() += number;

        Ok(())
    }

    fn get_total_support(&self) -> Result<u8, String> {
        panic!("Asked the total support of a LoC. They don't have any support.");
    }

    fn set_control_type(&mut self, _control_type: ControlTypes) -> Result<(), String> {
        panic!("Attempted to set the control type of a LoC. LoCs don't have control types. Something is wrong with the calling code.");
    }

    fn get_control_type(&self) -> Result<&ControlTypes, String> {
        panic!("Attempted to get the control type of a LoC. LoCs don't have control types. Something is wrong with the calling code.");
    }

    fn set_geographic_area(&mut self, geographic_area: GeographicAreas) -> Result<(), String> {
        self.geographic_area = geographic_area;

        Ok(())
    }

    fn get_geographic_area(&self) -> Result<&GeographicAreas, String> {
        Ok(&self.geographic_area)
    }

    fn is_adjacent_to_space(&self, space_identifier: SpaceIdentifiers) -> Result<bool, String> {
        Ok(self
            .adjacent_spaces
            .iter()
            .any(|adjacent_space| adjacent_space == &space_identifier))
    }

    fn get_terrain_type(&self) -> Result<&TerrainTypes, String> {
        // A LoC has no terrain type, and shouldn't ask of it.
        panic!("Asked the terrain type of a LoC. LoCs don't have any terrain type.");
    }

    fn get_population(&self) -> Result<u8, String> {
        // LoCs don't have a population, and it's wrong to ask.
        panic!("Asked the population of a LoC. LoCs don't have population!");
    }

    fn is_habitable(&self) -> Result<bool, String> {
        Ok(false)
    }

    fn has_terror(&self) -> Result<bool, String> {
        Ok(false)
    }

    fn set_terror(&mut self) -> Result<(), String> {
        panic!("Attempted to set terror on a LoC. They can only get sabotaged.");
    }

    fn unset_terror(&mut self) -> Result<(), String> {
        panic!("Attempted to unset terror on a LoC. They can only get sabotaged.");
    }

    fn has_sabotage(&self) -> Result<bool, String> {
        Ok(self.sabotage)
    }

    fn set_sabotage(&mut self) -> Result<(), String> {
        self.sabotage = true;

        Ok(())
    }

    fn unset_sabotage(&mut self) -> Result<(), String> {
        self.sabotage = false;

        Ok(())
    }

    fn get_adjacent_space_identifiers(&self) -> Result<&Vec<SpaceIdentifiers>, String> {
        Ok(&self.adjacent_spaces)
    }
}

impl LoC {
    pub fn new(
        space_identifier: SpaceIdentifiers,
        geographic_area: GeographicAreas,
        adjacent_spaces: Vec<SpaceIdentifiers>,
    ) -> LoC {
        LoC {
            space_identifier,
            forces: initialize_hashmap_of_forces(),
            geographic_area,
            adjacent_spaces,
            sabotage: false,
        }
    }
}
