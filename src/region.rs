
use super::support::Support;

pub struct Region {
    support: Support
}

impl Region {
    pub fn new() -> Region {
        Region {
            support: Support::Neutral
        }
    }
}

impl Region {
    pub fn support(&self) -> Support {
        self.support
    }

    pub fn set_support(&mut self, support: Support){
        self.support = support;
    }
}