use board::controls::Controls;
use board::map::Map;
use board::space::Space;
use board::support::SupportLevels;

#[derive(Debug)]
pub struct Track {
    aid: u8,
    nva_resources: u8,
    vc_resources: u8,
    arvn_resources: u8,
    trail: u8,
    nva_victory_marker: u8,
    us_victory_marker: u8,
    vc_victory_marker: u8,
    control_plus_patronage: u8,
    patronage: u8,
}

impl Track {
    pub fn new() -> Track {
        Track {
            aid: 0,
            nva_resources: 0,
            vc_resources: 0,
            arvn_resources: 0,
            trail: 0,
            nva_victory_marker: 0,
            us_victory_marker: 0,
            vc_victory_marker: 0,
            control_plus_patronage: 0,
            patronage: 0,
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

    pub fn get_vc_resources(&self) -> u8 {
        self.vc_resources
    }

    pub fn set_vc_resources(&mut self, new_vc_resources: u8) {
        self.vc_resources = new_vc_resources;
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

    pub fn get_patronage(&self) -> u8 {
        self.patronage
    }

    pub fn set_patronage(&mut self, new_patronage: u8) {
        self.patronage = new_patronage;
    }

    pub fn adjust_control_plus_patronage(&mut self, map: &Map) {
        // Calculated programatically.
        // A sum of all the population values of all the spaces that have COIN control.
        // To that you just add the patronage.
        let sum: u8 = map
            .get_spaces()
            .iter()
            .map(|space_entry| {
                // Look at the support of this space, and consider the population.
                // It won't matter the type of space, because those that wouldn't count
                // won't add to the sum because of their initial states in those fields.
                let (_, space) = space_entry;

                match space.get_control() {
                    Controls::Counterinsurgent => space.get_population_value(),
                    _ => 0,
                }
            })
            .sum();

        self.control_plus_patronage = sum + self.patronage
    }

    pub fn get_control_plus_patronage(&mut self) -> u8 {
        self.control_plus_patronage
    }

    pub fn get_nva_victory_marker(&self) -> u8 {
        self.nva_victory_marker
    }

    pub fn get_us_victory_marker(&self) -> u8 {
        self.us_victory_marker
    }

    pub fn get_vc_victory_marker(&self) -> u8 {
        self.vc_victory_marker
    }

    pub fn adjust_nva_victory_marker(&mut self, map: &Map) {
        // Total Population Controlled by the NVA plus the number of NVA Bases on the map

        let sum: u8 = map
            .get_spaces()
            .iter()
            .map(|space_entry| {
                let (_, space) = space_entry;

                match space.get_control() {
                    Controls::NVA => space.get_population_value(),
                    _ => 0,
                }
            })
            .sum();

        let number_of_nva_bases_on_map: u8 = map
            .get_spaces()
            .iter()
            .map(|space_entry| {
                let (_, space) = space_entry;

                space.get_number_of_nva_bases()
            })
            .sum();

        self.nva_victory_marker = sum + number_of_nva_bases_on_map;
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

    pub fn adjust_vc_victory_marker(&mut self, map: &Map) {
        // This is calculated programatically, but we need to know if a change has ocurred, because the
        // player will have to slide the corresponding marker in that case.
        // Total Opposition plus the number of VC Bases
        // Passive Opposition is 1xPop. Active Opposition count double Population for Total Support or Opposition.
        let sum: u8 = map
            .get_spaces()
            .iter()
            .map(|space_entry| {
                // Look at the support of this space, and consider the population.
                // It won't matter the type of space, because those that wouldn't count
                // won't add to the sum because of their initial states in those fields.
                let (_, space) = space_entry;

                let mut sum_for_space = 0;

                match space.get_support_level() {
                    SupportLevels::ActiveOpposition => {
                        sum_for_space += 2 * space.get_population_value()
                    }
                    SupportLevels::PassiveOpposition => {
                        sum_for_space += space.get_population_value()
                    }
                    _ => sum_for_space += 0,
                }

                // Must sum the VC bases in this location
                sum_for_space += space.get_number_of_vc_bases();

                sum_for_space
            })
            .sum();

        self.vc_victory_marker = sum;
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}
