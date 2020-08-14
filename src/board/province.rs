use board::controls::Controls;
use board::forces::Forces;
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
    forces: Forces,
}

impl Province {
    pub fn new(space_identifier: SpaceIdentifiers) -> Province {
        Province {
            space_identifier,
            population_value: 0,
            terrain_type: TerrainTypes::Lowland,
            support: Support::new(),
            control: Controls::Uncontrolled,
            forces: Forces::new(),
        }
    }
}

impl Space for Province {
    fn get_space_identifier(&self) -> SpaceIdentifiers {
        self.space_identifier
    }

    fn get_support_level(&self) -> SupportLevels {
        self.support.get_current_support_level()
    }

    fn set_support_level(&mut self, new_support_level: SupportLevels) {
        self.support.set_support_level(new_support_level);
    }

    fn shift_support_level_down(&mut self) {
        self.support.shift_support_level_down();
    }

    fn shift_support_level_up(&mut self) {
        self.support.shift_support_level_up();
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

    fn get_number_of_arvn_troops(&self) -> u8 {
        todo!()
    }

    fn set_number_of_arvn_troops(&mut self, _new_number_of_arvn_troops: u8) -> Result<(), String> {
        todo!()
    }

    fn get_number_of_nva_troops(&self) -> u8 {
        self.forces.get_number_of_nva_troops()
    }

    fn set_number_of_nva_troops(&mut self, new_number_of_nva_troops: u8) -> Result<(), String> {
        self.forces.set_number_of_nva_troops(new_number_of_nva_troops);

        Ok(())
    }

    fn are_there_nva_bases(&self) -> bool {
        self.forces.get_number_of_nva_bases() > 0
    }

    fn are_there_vc_bases(&self) -> bool {
        self.forces.get_number_of_vc_bases() > 0
    }

    fn get_number_of_nva_bases(&self) -> u8 {
        self.forces.get_number_of_nva_bases()
    }

    fn set_number_of_nva_bases(&mut self, new_number_of_nva_bases: u8) {
        self.forces.set_number_of_nva_bases(new_number_of_nva_bases);
        self.adjust_control();
    }

    fn get_number_of_underground_nva_guerrillas(&self) -> u8 {
        self.forces.get_number_of_underground_nva_guerrillas()
    }

    fn set_number_of_underground_nva_guerrillas(
        &mut self,
        new_number_of_underground_nva_guerrillas: u8,
    ) {
        self.forces
            .set_number_of_underground_nva_guerrillas(new_number_of_underground_nva_guerrillas);
        self.adjust_control();
    }

    fn get_number_of_underground_special_forces_irregulars(&self) -> u8 {
        self.forces
            .get_number_of_underground_special_forces_irregulars()
    }

    fn set_number_of_underground_special_forces_irregulars(
        &mut self,
        new_number_of_undergound_special_forces_irregulars: u8,
    ) {
        self.forces
            .set_number_of_underground_special_forces_irregulars(
                new_number_of_undergound_special_forces_irregulars,
            );
        self.adjust_control();
    }

    fn get_number_of_us_troops(&self) -> u8 {
        self.forces.get_number_of_us_troops()
    }

    fn set_number_of_us_troops(&mut self, new_number_of_us_troops: u8) {
        self.forces.set_number_of_us_troops(new_number_of_us_troops);
        self.adjust_control();
    }

    fn get_number_of_active_vc_guerrillas(&self) -> u8 {
        self.forces.get_active_vc_guerrillas()
    }
    fn set_number_of_active_vc_guerrillas(&mut self, new_number_of_active_vc_guerrillas: u8) {
        self.forces
            .set_active_vc_guerrillas(new_number_of_active_vc_guerrillas);
        self.adjust_control();
    }

    fn get_number_of_underground_vc_guerrillas(&self) -> u8 {
        self.forces.get_underground_vc_guerrillas()
    }

    fn set_number_of_underground_vc_guerrillas(
        &mut self,
        new_number_of_underground_vc_guerrillas: u8,
    ) {
        self.forces
            .set_underground_vc_guerrillas(new_number_of_underground_vc_guerrillas);
        self.adjust_control();
    }

    fn get_total_number_of_us_pieces(&self) -> u8 {
        self.forces.get_total_number_of_us_pieces()
    }

    fn get_total_number_of_vc_pieces(&self) -> u8 {
        self.forces.get_total_number_of_vc_pieces()
    }

    fn get_number_of_vc_bases(&self) -> u8 {
        self.forces.get_number_of_vc_bases()
    }

    fn set_number_of_vc_bases(&mut self, new_number_of_vc_bases: u8) {
        self.forces.set_number_of_vc_bases(new_number_of_vc_bases);
        self.adjust_control();
    }

    fn get_number_of_us_bases(&self) -> u8 {
        self.forces.get_number_of_us_bases()
    }

    fn set_number_of_us_bases(&mut self, new_number_of_us_bases: u8) {
        self.forces.set_number_of_us_bases(new_number_of_us_bases)
    }

    fn get_total_number_of_nva_pieces(&self) -> u8{
        todo!()
    }

    fn get_total_number_of_arvn_pieces(&self) -> u8{
        todo!()
    }

    fn adjust_control(&mut self) {
        // COIN control: US + ARVN forces outnumber VC + NVA ones
        if (self.forces.get_total_number_of_us_pieces() + self.forces.get_total_number_of_arvn_pieces()) > (self.forces.get_total_number_of_vc_pieces() + self.forces.get_total_number_of_nva_pieces())
        {
            self.control = Controls::Counterinsurgent;
        }
        else if self.forces.get_total_number_of_nva_pieces() > (self.forces.get_total_number_of_us_pieces() + self.forces.get_total_number_of_vc_pieces() + self.forces.get_total_number_of_arvn_pieces()){
            // NVA control if more NVA pieces present than all the other factions combined.
            self.control = Controls::NVA;
        }
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
