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
use game_definitions::terrain_types::TerrainTypes;

#[enum_dispatch]
#[allow(clippy::too_many_arguments)]
pub trait Space {
    fn get_space_identifier(&self) -> Result<SpaceIdentifiers, String>;
    fn set_forces(&mut self, forces: Forces, value: u8) -> Result<(), String>;
    fn get_forces(&self, forces: Forces) -> Result<u8, String>;
    fn get_support_level(&self) -> Result<&SupportLevels, String>;
    fn set_support_level(&mut self, support_level: SupportLevels) -> Result<(), String>;
    fn get_total_support(&self) -> Result<u8, String>;
    fn reduce_forces(&mut self, forces: &Forces, number: u8) -> Result<(), String>;
    fn increase_forces(&mut self, forces: &Forces, number: u8) -> Result<(), String>;
    fn set_control_type(&mut self, control_type: ControlTypes) -> Result<(), String>;
    fn get_control_type(&self) -> Result<&ControlTypes, String>;
    fn set_geographic_area(&mut self, geographic_area: GeographicAreas) -> Result<(), String>;
    fn get_geographic_area(&self) -> Result<&GeographicAreas, String>;
    fn get_adjacent_space_identifiers(&self) -> Result<&Vec<SpaceIdentifiers>, String>;
    fn is_adjacent_to_space(&self, space_identifier: SpaceIdentifiers) -> Result<bool, String>;
    fn get_terrain_type(&self) -> Result<&TerrainTypes, String>;
    fn get_population(&self) -> Result<u8, String>;
    fn is_habitable(&self) -> Result<bool, String>;
    fn has_terror(&self) -> Result<bool, String>;
    fn set_terror(&mut self) -> Result<(), String>;
    fn unset_terror(&mut self) -> Result<(), String>;
    fn has_sabotage(&self) -> Result<bool, String>;
    fn set_sabotage(&mut self) -> Result<(), String>;
    fn unset_sabotage(&mut self) -> Result<(), String>;
}

#[enum_dispatch(Space)]
#[derive(Debug, Clone)]
pub enum Spaces {
    Province,
    City,
    LoC,
}
