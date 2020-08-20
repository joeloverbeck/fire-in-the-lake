use board::domain::initialize_hashmap_of_forces::initialize_hashmap_of_forces;
use game_definitions::control_types::ControlTypes;
use game_definitions::forces::Forces;
use game_definitions::geographic_area::GeographicArea;
use game_definitions::support_levels::SupportLevels;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OccupableSpace {
    forces: HashMap<Forces, u8>,
    control_type: ControlTypes,
    support_level: SupportLevels,
    geographic_area: GeographicArea,
}

impl OccupableSpace {
    pub fn new(geographic_area: GeographicArea) -> OccupableSpace {
        OccupableSpace {
            forces: initialize_hashmap_of_forces(),
            control_type: ControlTypes::Uncontrolled,
            support_level: SupportLevels::Neutral,
            geographic_area,
        }
    }

    pub fn set_forces(&mut self, forces: Forces, value: u8) -> Result<(), String> {
        *self.forces.get_mut(&forces).unwrap() = value;

        Ok(())
    }

    pub fn get_forces(&self, forces: Forces) -> Result<u8, String> {
        Ok(*self.forces.get(&forces).unwrap())
    }

    pub fn get_support_level(&self) -> Result<&SupportLevels, String> {
        Ok(&self.support_level)
    }

    pub fn set_support_level(&mut self, support_level: SupportLevels) -> Result<(), String> {
        self.support_level = support_level;

        Ok(())
    }

    pub fn reduce_forces(&mut self, forces: &Forces, number: u8) -> Result<(), String> {
        *self.forces.get_mut(&forces).unwrap() -= number;

        Ok(())
    }

    pub fn increase_forces(&mut self, forces: &Forces, number: u8) -> Result<(), String> {
        *self.forces.get_mut(&forces).unwrap() += number;

        Ok(())
    }

    pub fn set_control_type(&mut self, control_type: ControlTypes) -> Result<(), String> {
        self.control_type = control_type;

        Ok(())
    }

    pub fn get_control_type(&self) -> Result<&ControlTypes, String> {
        Ok(&self.control_type)
    }

    pub fn set_geographic_area(&mut self, geographic_area: GeographicArea) -> Result<(), String> {
        self.geographic_area = geographic_area;

        Ok(())
    }

    pub fn get_geographic_area(&self) -> Result<&GeographicArea, String> {
        Ok(&self.geographic_area)
    }
}
