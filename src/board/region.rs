
use board::space_identifiers::SpaceIdentifiers;
use board::support::Support;

#[derive(Debug)]
pub struct Region {
    region_identifier: SpaceIdentifiers,
    support: Support
}

impl Region {
    pub fn region_identifier(&self) -> SpaceIdentifiers {
        self.region_identifier
    }

    pub fn new(region_identifier: SpaceIdentifiers) -> Region {
        Region {
            region_identifier: region_identifier,
            support: Support::Neutral
        }
    }

    pub fn support(&self) -> Support {
        self.support
    }

    pub fn set_support(&mut self, support: Support){
        self.support = support;
    }
}