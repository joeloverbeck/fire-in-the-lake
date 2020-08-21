extern crate enum_dispatch;
use self::enum_dispatch::enum_dispatch;
use board::domain::city::City;
use board::domain::loc::LoC;
use board::domain::province::Province;
use game_definitions::control_types::ControlTypes;
use game_definitions::forces::Forces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::support_levels::SupportLevels;

#[enum_dispatch]
#[allow(clippy::too_many_arguments)]
pub trait Space {
    fn set_forces(&mut self, forces: Forces, value: u8) -> Result<(), String>;
    fn get_forces(&self, forces: Forces) -> Result<u8, String>;
    fn get_support_level(&self) -> Result<&SupportLevels, String>;
    fn set_support_level(&mut self, support_level: SupportLevels) -> Result<(), String>;
    fn reduce_forces(&mut self, forces: &Forces, number: u8) -> Result<(), String>;
    fn increase_forces(&mut self, forces: &Forces, number: u8) -> Result<(), String>;
    fn set_control_type(&mut self, control_type: ControlTypes) -> Result<(), String>;
    fn get_control_type(&self) -> Result<&ControlTypes, String>;
    fn set_geographic_area(&mut self, geographic_area: GeographicAreas) -> Result<(), String>;
    fn get_geographic_area(&self) -> Result<&GeographicAreas, String>;
    fn is_adjacent_to_space(&self, space_identifier: SpaceIdentifiers) -> Result<bool, String>;
}

#[enum_dispatch(Space)]
#[derive(Debug, Clone)]
pub enum Spaces {
    Province,
    City,
    LoC,
}
