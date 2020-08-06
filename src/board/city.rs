use board::support::SupportLevels;
use board::support::Support;
use board::space_identifiers::SpaceIdentifiers;
use board::space::Space;
use board::terrain_types::TerrainTypes;
use board::controls::Controls;

#[derive(Debug)]
pub struct City {
    space_identifier: SpaceIdentifiers,
    population_value: u8,
    terrain_type: TerrainTypes,
    support: Support,
    control: Controls      
}

impl City {
    pub fn new(space_identifier: SpaceIdentifiers) -> City{
        City {
            space_identifier: space_identifier,
            population_value: 0,
            terrain_type: TerrainTypes::City,            
            support: Support::new(),
            control: Controls::Uncontrolled  
        }
    }
}

impl Space for City {
    fn get_space_identifier(&self) -> SpaceIdentifiers{
        self.space_identifier
    }

    fn get_current_support_level(&self) -> SupportLevels {
        todo!()
    }

    fn set_support_level(&mut self, new_support_level: SupportLevels) {
        todo!()
    }

    fn shift_support_level_down(&mut self) {
        todo!()
    }

    fn get_terrain_type(&self) -> TerrainTypes {
        // Terrain type for a City shouldn't exist; it's an artifact of the inability
        // to do the black magic of other languages in Rust. In any case, it should always
        // return City.
        TerrainTypes::City
    }

    fn set_terrain_type(&mut self, new_terrain_type: TerrainTypes) {
        todo!()
    }

    fn get_population_value(&self) -> u8{
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
        todo!()
    }
}