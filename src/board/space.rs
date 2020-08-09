use board::space_identifiers::SpaceIdentifiers;

use board::city::City;
use board::controls::Controls;
use board::line_of_communication::LineOfCommunication;
use board::province::Province;
use board::support::SupportLevels;
use board::terrain_types::TerrainTypes;

extern crate enum_dispatch;
use self::enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Space {
    fn get_space_identifier(&self) -> SpaceIdentifiers;
    fn get_support_level(&self) -> SupportLevels;
    fn set_support_level(&mut self, new_support_level: SupportLevels);
    fn shift_support_level_down(&mut self);
    fn shift_support_level_up(&mut self);
    fn get_terrain_type(&self) -> TerrainTypes;
    fn set_terrain_type(&mut self, new_terrain_type: TerrainTypes);
    fn get_population_value(&self) -> u8;
    fn set_population_value(&mut self, new_population_value: u8) -> Result<(), String>;
    fn get_control(&self) -> Controls;
    fn set_control(&mut self, new_control: Controls);
    fn get_number_of_arvn_troops(&self) -> u8;
    fn set_number_of_arvn_troops(&mut self, new_number_of_arvn_troops: u8) -> Result<(), String>;
    fn are_there_nva_bases(&self) -> bool;
    fn set_number_of_nva_bases(&mut self, new_number_of_nva_bases: u8);
    fn get_number_of_underground_nva_guerrillas(&self) -> u8;
    fn set_number_of_underground_nva_guerrillas(
        &mut self,
        new_number_of_underground_nva_guerrillas: u8,
    );
}

#[enum_dispatch(Space)]
#[derive(Debug)]
pub enum Spaces {
    City,
    Province,
    LineOfCommunication,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_should_be_able_to_create_a_hashmap_of_all_possible_spaces() -> Result<(), String> {
        let city = City::new(SpaceIdentifiers::Saigon).into();
        let province = Province::new(SpaceIdentifiers::Saigon).into();
        let line_of_communication = LineOfCommunication::new(SpaceIdentifiers::Saigon).into();

        let mut registered_spaces: HashMap<SpaceIdentifiers, Spaces> = HashMap::new();

        registered_spaces.insert(SpaceIdentifiers::Saigon, city);
        registered_spaces.insert(SpaceIdentifiers::Saigon, province);
        registered_spaces.insert(SpaceIdentifiers::Saigon, line_of_communication);

        if let Some(retrieved_space) = registered_spaces.get(&SpaceIdentifiers::Saigon) {
            assert!(retrieved_space.get_space_identifier() == SpaceIdentifiers::Saigon);
        } else {
            return Err(String::from(
                "Could not retrieve the inserted space from the hashmap",
            ));
        }

        Ok(())
    }
}
