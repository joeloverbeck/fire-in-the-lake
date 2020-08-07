use board::controls::Controls;
use board::space::Space;
use board::space_identifiers::SpaceIdentifiers;
use board::support::Support;
use board::support::SupportLevels;
use board::terrain_types::TerrainTypes;

#[derive(Debug)]
pub struct Province {
    space_identifier: SpaceIdentifiers,
    population_value: u8,
    terrain_type: TerrainTypes,
    support: Support,
    control: Controls,
}

impl Province {
    pub fn new(space_identifier: SpaceIdentifiers) -> Province {
        Province {
            space_identifier,
            population_value: 0,
            terrain_type: TerrainTypes::Lowland,
            support: Support::new(),
            control: Controls::Uncontrolled,
        }
    }
}

impl Space for Province {
    fn get_space_identifier(&self) -> SpaceIdentifiers {
        self.space_identifier
    }

    fn get_current_support_level(&self) -> SupportLevels {
        self.support.get_current_support_level()
    }

    fn set_support_level(&mut self, new_support_level: SupportLevels) {
        self.support.set_support_level(new_support_level);
    }

    fn shift_support_level_down(&mut self) {
        self.support.shift_support_level_down();
    }

    fn get_terrain_type(&self) -> TerrainTypes {
        self.terrain_type
    }

    fn set_terrain_type(&mut self, new_terrain_type: TerrainTypes) {
        self.terrain_type = new_terrain_type;
    }

    fn get_population_value(&self) -> u8 {
        self.population_value
    }

    fn set_population_value(&mut self, new_population_value: u8) -> Result<(), String> {
        self.population_value = new_population_value;

        Ok(())
    }

    fn get_control(&self) -> Controls {
        self.control
    }

    fn set_control(&mut self, new_control: Controls) {
        self.control = new_control;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use board::space::Spaces;

    #[test]
    fn test_should_be_able_to_retrieve_expected_terrain_type_of_province() -> Result<(), String> {
        let mut space: Spaces = Province::new(SpaceIdentifiers::KienGiangAnXuyen).into();

        space.set_terrain_type(TerrainTypes::Highland);

        assert_eq!(space.get_terrain_type(), TerrainTypes::Highland);

        Ok(())
    }

    #[test]
    fn test_when_creating_a_province_with_a_set_space_identifier_when_asking_for_the_identifier_it_should_be_the_expected_one(
    ) -> Result<(), String> {
        let mut space: Spaces = Province::new(SpaceIdentifiers::KienGiangAnXuyen).into();

        assert_eq!(
            space.get_space_identifier(),
            SpaceIdentifiers::KienGiangAnXuyen
        );

        Ok(())
    }

    #[test]
    fn test_should_be_able_to_get_the_population_value_of_the_province() -> Result<(), String> {
        let mut space: Spaces = Province::new(SpaceIdentifiers::KienGiangAnXuyen).into();

        space.set_population_value(1);

        assert_eq!(space.get_population_value(), 1);

        Ok(())
    }

    #[test]
    fn test_should_be_able_to_get_who_is_in_control_of_the_province() -> Result<(), String> {
        let mut space: Spaces = Province::new(SpaceIdentifiers::KienGiangAnXuyen).into();

        space.set_control(Controls::NVA);

        assert_eq!(space.get_control(), Controls::NVA);

        Ok(())
    }
}
