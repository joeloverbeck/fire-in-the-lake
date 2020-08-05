
use board::regions::Regions;
use board::support::Support;

#[derive(Debug)]
pub struct Region {
    region_identifier: Regions,
    support: Support
}

impl Region {
    pub fn region_identifier(&self) -> Regions {
        self.region_identifier
    }

    pub fn new(region_identifier: Regions) -> Region {
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