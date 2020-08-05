use board::space_identifiers::SpaceIdentifiers;
use board::space::Space;


pub struct Province {
    
}

impl Province {
    pub fn new() -> Province {
        Province {
            
        }
    }
}

impl Space for Province {
    fn get_space_identifier(&self) -> SpaceIdentifiers{
        SpaceIdentifiers::Saigon
    }
}