use board::space_identifiers::SpaceIdentifiers;
use board::space::Space;
use board::support::SupportLevels;
use board::support::Support;

#[derive(Debug)]
pub struct Province {
    support: Support
}

impl Province {
    pub fn new(space_identifier: SpaceIdentifiers) -> Province {
        Province {
            support: Support::new()
        }
    }
}

impl Space for Province {
    fn get_space_identifier(&self) -> SpaceIdentifiers{
        SpaceIdentifiers::Saigon
    }

    fn get_current_support_level(&self) -> SupportLevels {
        self.support.get_current_support_level()
    }

    fn set_support_level(&mut self, new_support_level: SupportLevels) {
        self.support.set_support_level(new_support_level);
    }

    fn shift_support_level_down(&mut self) {
        self.support.shift_support_level_down();
    }
}