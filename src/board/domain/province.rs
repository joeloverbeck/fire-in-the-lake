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
pub struct Province {
    space_identifier: SpaceIdentifiers,
    population: u8,
    forces: HashMap<Forces, u8>,
    control_type: ControlTypes,
    support_level: SupportLevels,
    geographic_area: GeographicAreas,
    terrain_type: TerrainTypes,
    adjacent_spaces: Vec<SpaceIdentifiers>,
    terror: bool,
}

impl Space for Province {
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
        Ok(&self.support_level)
    }

    fn set_support_level(&mut self, support_level: SupportLevels) -> Result<(), String> {
        self.support_level = support_level;

        Ok(())
    }

    fn get_total_support(&self) -> Result<u8, String> {
        match self.support_level {
            SupportLevels::ActiveSupport => Ok(self.population * 2),
            SupportLevels::PassiveSupport => Ok(self.population),
            _ => Ok(0),
        }
    }

    fn reduce_forces(&mut self, forces: &Forces, number: u8) -> Result<(), String> {
        *self.forces.get_mut(&forces).unwrap() -= number;

        Ok(())
    }

    fn increase_forces(&mut self, forces: &Forces, number: u8) -> Result<(), String> {
        *self.forces.get_mut(&forces).unwrap() += number;

        Ok(())
    }

    fn set_control_type(&mut self, control_type: ControlTypes) -> Result<(), String> {
        self.control_type = control_type;

        Ok(())
    }

    fn get_control_type(&self) -> Result<&ControlTypes, String> {
        Ok(&self.control_type)
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
        Ok(&self.terrain_type)
    }

    fn get_population(&self) -> Result<u8, String> {
        Ok(self.population)
    }

    fn is_habitable(&self) -> Result<bool, String> {
        Ok(true)
    }

    fn has_terror(&self) -> Result<bool, String> {
        Ok(self.terror)
    }

    fn set_terror(&mut self) -> Result<(), String> {
        self.terror = true;

        Ok(())
    }

    fn unset_terror(&mut self) -> Result<(), String> {
        self.terror = false;

        Ok(())
    }

    fn has_sabotage(&self) -> Result<bool, String> {
        Ok(false)
    }

    fn set_sabotage(&mut self) -> Result<(), String> {
        panic!("Attempted to set sabotage on a province. Only LoCs can be sabotaged.");
    }

    fn unset_sabotage(&mut self) -> Result<(), String> {
        panic!("Attempted to unset sabotage on a province. Only LoCs can be sabotaged.");
    }
}

impl Province {
    pub fn new(
        space_identifier: SpaceIdentifiers,
        population: u8,
        geographic_area: GeographicAreas,
        terrain_type: TerrainTypes,
        adjacent_spaces: Vec<SpaceIdentifiers>,
    ) -> Province {
        Province {
            space_identifier,
            population,
            forces: initialize_hashmap_of_forces(),
            control_type: ControlTypes::Uncontrolled,
            support_level: SupportLevels::Neutral,
            geographic_area,
            terrain_type,
            adjacent_spaces,
            terror: false,
        }
    }
}
