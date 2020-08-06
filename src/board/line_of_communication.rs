use board::space_identifiers::SpaceIdentifiers;
use board::space::Space;
use board::support::SupportLevels;
use board::space::Spaces;
use board::terrain_types::TerrainTypes;
use board::controls::Controls;


#[derive(Debug)]
pub struct LineOfCommunication {

}

impl LineOfCommunication {
    pub fn new() -> LineOfCommunication {
        LineOfCommunication {

        }
    }
}

impl Space for LineOfCommunication {
    fn get_space_identifier(&self) -> SpaceIdentifiers{
        SpaceIdentifiers::Saigon
    }

    fn get_current_support_level(&self) -> SupportLevels {
        // The support level of a LoC is always Neutral.
        SupportLevels::Neutral
    }

    fn set_support_level(&mut self, new_support_level: SupportLevels) {
        // No support level changing in LoCs.
        panic!("Attempted to set the support level in a LoC. That should never happen.")
    }

    fn shift_support_level_down(&mut self) {
        // There's no support shifting in LoCs.
        panic!("Attempted to shift the support level in a LoC. That should never happen.")
    }

    fn get_terrain_type(&self) -> TerrainTypes {
        todo!()
    }

    fn set_terrain_type(&mut self, new_terrain_type: TerrainTypes) {
        todo!()
    }

    fn get_population_value(&self) -> u8 {
        todo!()
    }

    fn set_population_value(&mut self, new_population_value: u8) -> Result<(), String> {
        todo!()
    }

    fn get_control(&self) -> Controls {
        todo!()
    }

    fn set_control(&mut self, new_control: Controls) {
        todo!()
    }
}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_the_support_level_of_a_loc_is_always_neutral() -> Result<(), String> {
        let loc: Spaces = LineOfCommunication::new().into();

        assert_eq!(loc.get_current_support_level(), SupportLevels::Neutral);
        
        Ok(())
    }

}