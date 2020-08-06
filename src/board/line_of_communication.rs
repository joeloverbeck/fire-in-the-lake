use board::space_identifiers::SpaceIdentifiers;
use board::space::Space;
use board::support::SupportLevels;


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
        todo!()
    }

    fn set_support_level(&mut self, new_support_level: SupportLevels) {
        todo!()
    }

    fn shift_support_level_down(&mut self) {
        todo!()
    }
}