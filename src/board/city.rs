use board::space_identifiers::SpaceIdentifiers;
use board::space::Space;

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
}