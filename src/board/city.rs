use board::support::SupportLevels;
use board::space_identifiers::SpaceIdentifiers;
use board::space::Space;
use board::terrain_types::TerrainTypes;

#[derive(Debug)]
pub struct City {
    
}

impl City {
    pub fn new() -> City{
        City {
            
        }
    }
}

impl Space for City {
    fn get_space_identifier(&self) -> SpaceIdentifiers{
        SpaceIdentifiers::Saigon
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
        todo!()
    }

    fn set_terrain_type(&mut self, new_terrain_type: TerrainTypes) {
        todo!()
    }
}