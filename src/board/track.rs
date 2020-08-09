use board::map::Map;
use board::space::Space;
use board::support::SupportLevels;

#[derive(Debug)]
pub struct Track {
    aid: u8,
    nva_resources: u8,
    arvn_resources: u8,
    trail: u8,
    us_victory_marker: u8,
}

impl Track {
    pub fn new() -> Track {
        Track {
            aid: 0,
            nva_resources: 0,
            arvn_resources: 0,
            trail: 0,
            us_victory_marker: 0,
        }
    }

    pub fn get_aid(&self) -> u8 {
        self.aid
    }

    pub fn set_aid(&mut self, new_aid: u8) {
        self.aid = new_aid
    }

    pub fn get_nva_resources(&self) -> u8 {
        self.nva_resources
    }

    pub fn set_nva_resources(&mut self, new_nva_resources: u8) {
        self.nva_resources = new_nva_resources;
    }

    pub fn get_arvn_resources(&self) -> u8 {
        self.arvn_resources
    }

    pub fn set_arvn_resources(&mut self, new_arvn_resources: u8) {
        self.arvn_resources = new_arvn_resources;
    }

    pub fn get_trail(&self) -> u8 {
        self.trail
    }

    pub fn set_trail(&mut self, new_trail: u8) {
        self.trail = new_trail;
    }

    pub fn adjust_us_victory_marker(&mut self, map: &Map) {
        // This is calculated programatically, but we need to know if a change has ocurred, because the
        // player will have to slide the corresponding marker in that case.
        // Total Support (1.6.2) plus the number of Troops and Bases in the US Available Forces box
        // Passive Support is 1xPop. Active Support or Opposition count double Population for Total Support or Opposition.
        let sum: u8 = map
            .get_spaces()
            .iter()
            .map(|space_entry| {
                // Look at the support of this space, and consider the population.
                // It won't matter the type of space, because those that wouldn't count
                // won't add to the sum because of their initial states in those fields.
                let (_, space) = space_entry;

                match space.get_support_level() {
                    SupportLevels::ActiveSupport => 2 * space.get_population_value(),
                    SupportLevels::PassiveSupport => space.get_population_value(),
                    _ => 0,
                }
            })
            .sum();

        self.us_victory_marker = sum;
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}
