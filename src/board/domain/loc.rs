use board::domain::initialize_hashmap_of_forces::initialize_hashmap_of_forces;
use board::domain::space::Space;
use game_definitions::control_types::ControlTypes;
use game_definitions::forces::Forces;
use game_definitions::geographic_area::GeographicArea;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::support_levels::SupportLevels;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LoC {
    forces: HashMap<Forces, u8>,
    geographic_area: GeographicArea,
    adjacent_spaces: Vec<SpaceIdentifiers>,
}

impl Space for LoC {
    fn set_forces(&mut self, forces: Forces, value: u8) -> Result<(), String> {
        *self.forces.get_mut(&forces).unwrap() = value;

        Ok(())
    }

    fn get_forces(&self, forces: Forces) -> Result<u8, String> {
        Ok(*self.forces.get(&forces).unwrap())
    }

    fn get_support_level(&self) -> Result<&SupportLevels, String> {
        panic!("Asked an LoC's support level. LoCs don't have support levels. Something is wrong with the calling code.");
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

    fn set_control_type(&mut self, _control_type: ControlTypes) -> Result<(), String> {
        panic!("Attempted to set the control type of a LoC. LoCs don't have control types. Something is wrong with the calling code.");
    }

    fn get_control_type(&self) -> Result<&ControlTypes, String> {
        panic!("Attempted to get the control type of a LoC. LoCs don't have control types. Something is wrong with the calling code.");
    }

    fn set_geographic_area(&mut self, geographic_area: GeographicArea) -> Result<(), String> {
        self.geographic_area = geographic_area;

        Ok(())
    }

    fn get_geographic_area(&self) -> Result<&GeographicArea, String> {
        Ok(&self.geographic_area)
    }

    fn is_adjacent_to_space(&self, space_identifier: SpaceIdentifiers) -> Result<bool, String> {
        Ok(self
            .adjacent_spaces
            .iter()
            .any(|adjacent_space| adjacent_space == &space_identifier))
    }
}

impl LoC {
    pub fn new(geographic_area: GeographicArea, adjacent_spaces: Vec<SpaceIdentifiers>) -> LoC {
        LoC {
            forces: initialize_hashmap_of_forces(),
            geographic_area,
            adjacent_spaces,
        }
    }
}
