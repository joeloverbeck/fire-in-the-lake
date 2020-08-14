use board::controls::Controls;
use board::space::Space;
use board::space_identifiers::SpaceIdentifiers;
use board::support::SupportLevels;
use board::terrain_types::TerrainTypes;

#[derive(Debug)]
pub struct LineOfCommunication {
    space_identifier: SpaceIdentifiers,
    population_value: u8,
}

impl LineOfCommunication {
    pub fn new(space_identifier: SpaceIdentifiers) -> LineOfCommunication {
        LineOfCommunication {
            space_identifier,
            population_value: 0,
        }
    }
}

impl Space for LineOfCommunication {
    fn get_space_identifier(&self) -> SpaceIdentifiers {
        self.space_identifier
    }

    fn get_support_level(&self) -> SupportLevels {
        // The support level of a LoC is always Neutral.
        SupportLevels::Neutral
    }

    fn set_support_level(&mut self, _new_support_level: SupportLevels) {
        // No support level changing in LoCs.
        panic!("Attempted to set the support level in a LoC. That should never happen.")
    }

    fn shift_support_level_down(&mut self) {
        // There's no support shifting in LoCs.
        panic!("Attempted to shift the support level in a LoC. That should never happen.")
    }

    fn shift_support_level_up(&mut self) {
        todo!()
    }

    fn get_terrain_type(&self) -> TerrainTypes {
        // The terrain type for a Line of Communication is always an LoC
        TerrainTypes::LoC
    }

    fn set_terrain_type(&mut self, _new_terrain_type: TerrainTypes) {
        // The terrain type is always LoC
    }

    fn get_population_value(&self) -> u8 {
        self.population_value
    }

    fn set_population_value(&mut self, new_population_value: u8) -> Result<(), String> {
        self.population_value = new_population_value;

        Ok(())
    }

    fn get_control(&self) -> Controls {
        // LoCs are always uncontrolled
        Controls::Uncontrolled
    }

    fn set_control(&mut self, _new_control: Controls) {
        todo!()
    }

    fn get_number_of_arvn_troops(&self) -> u8 {
        todo!()
    }

    fn set_number_of_arvn_troops(&mut self, _new_number_of_arvn_troops: u8) -> Result<(), String> {
        todo!()
    }

    fn get_number_of_nva_troops(&self) -> u8 {
        todo!()
    }

    fn set_number_of_nva_troops(&mut self, _new_number_of_nva_troops: u8) -> Result<(), String> {
        todo!()
    }

    fn get_number_of_nva_bases(&self) -> u8 {
        0
    }

    fn are_there_nva_bases(&self) -> bool {
        todo!()
    }

    fn set_number_of_nva_bases(&mut self, _new_number_of_nva_bases: u8) {
        todo!()
    }

    fn get_number_of_underground_nva_guerrillas(&self) -> u8 {
        todo!()
    }

    fn set_number_of_underground_nva_guerrillas(
        &mut self,
        _new_number_of_underground_nva_guerrillas: u8,
    ) {
        todo!()
    }

    fn get_number_of_underground_special_forces_irregulars(&self) -> u8 {
        todo!()
    }

    fn set_number_of_underground_special_forces_irregulars(
        &mut self,
        _new_number_of_undergound_special_forces_irregulars: u8,
    ) {
        todo!()
    }

    fn get_number_of_us_troops(&self) -> u8 {
        todo!()
    }

    fn set_number_of_us_troops(&mut self, _new_number_of_us_troops: u8) {
        todo!()
    }

    fn get_number_of_active_vc_guerrillas(&self) -> u8 {
        todo!()
    }
    fn set_number_of_active_vc_guerrillas(&mut self, _new_number_of_active_vc_guerrillas: u8) {
        todo!()
    }

    fn get_number_of_underground_vc_guerrillas(&self) -> u8 {
        todo!()
    }

    fn are_there_vc_bases(&self) -> bool {
        todo!()
    }

    fn set_number_of_underground_vc_guerrillas(
        &mut self,
        _new_number_of_underground_vc_guerrillas: u8,
    ) {
        todo!()
    }

    fn get_total_number_of_us_pieces(&self) -> u8 {
        todo!()
    }

    fn get_total_number_of_vc_pieces(&self) -> u8 {
        todo!()
    }

    fn get_number_of_vc_bases(&self) -> u8 {
        // Can't have bases in a LoC.
        0
    }

    fn set_number_of_vc_bases(&mut self, _new_number_of_vc_bases: u8) {
        todo!()
    }

    fn get_number_of_us_bases(&self) -> u8 {
        todo!()
    }

    fn set_number_of_us_bases(&mut self, _new_number_of_us_bases: u8) {
        todo!()
    }

    fn get_total_number_of_nva_pieces(&self) -> u8 {
        todo!()
    }

    fn get_total_number_of_arvn_pieces(&self) -> u8 {
        todo!()
    }

    fn adjust_control(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use board::space::Spaces;

    #[test]
    fn test_the_support_level_of_a_loc_is_always_neutral() -> Result<(), String> {
        let loc: Spaces = LineOfCommunication::new(SpaceIdentifiers::CanTho).into();

        assert_eq!(loc.get_support_level(), SupportLevels::Neutral);

        Ok(())
    }
}
