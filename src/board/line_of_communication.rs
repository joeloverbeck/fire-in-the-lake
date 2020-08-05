use board::space_identifiers::SpaceIdentifiers;
use board::space::Space;


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
}