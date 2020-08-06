use board::space_identifiers::SpaceIdentifiers;
use board::space::Space;
use board::support::SupportLevels;
use board::support::Support;
use board::terrain_types::TerrainTypes;

#[derive(Debug)]
pub struct Province {
    support: Support,
    terrain_type: TerrainTypes
}

impl Province {
    pub fn new(space_identifier: SpaceIdentifiers) -> Province {
        Province {
            support: Support::new(),
            terrain_type: TerrainTypes::Highland
        }
    }
}

impl Space for Province {
    fn get_space_identifier(&self) -> SpaceIdentifiers{
        SpaceIdentifiers::Saigon
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

}