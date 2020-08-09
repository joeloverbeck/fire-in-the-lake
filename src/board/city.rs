use board::controls::Controls;
use board::forces::Forces;
use board::space::Space;
use board::space_identifiers::SpaceIdentifiers;
use board::support::Support;
use board::support::SupportLevels;
use board::terrain_types::TerrainTypes;

#[derive(Debug)]
pub struct City {
    space_identifier: SpaceIdentifiers,
    population_value: u8,
    terrain_type: TerrainTypes,
    support: Support,
    control: Controls,
    forces: Forces,
}

impl City {
    pub fn new(space_identifier: SpaceIdentifiers) -> City {
        City {
            space_identifier,
            population_value: 0,
            terrain_type: TerrainTypes::City,
            support: Support::new(),
            control: Controls::Uncontrolled,
            forces: Forces::new(),
        }
    }
}

impl Space for City {
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
        // Terrain type for a City shouldn't exist; it's an artifact of the inability
        // to do the black magic of other languages in Rust. In any case, it should always
        // return City.
        TerrainTypes::City
    }

    fn set_terrain_type(&mut self, _new_terrain_type: TerrainTypes) {
        // A city is always a City
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

    fn set_control(&mut self, _new_control: Controls) {
        todo!()
    }

    fn get_number_of_arvn_troops(&self) -> u8 {
        self.forces.get_number_of_arvn_troops()
    }

    fn set_number_of_arvn_troops(&mut self, new_number_of_arvn_troops: u8) -> Result<(), String> {
        self.forces
            .set_number_of_arvn_troops(new_number_of_arvn_troops)?;

        Ok(())
    }
}
