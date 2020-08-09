use board::map::Map;
use board::track::Track;
use decision_making::player::Player;

#[derive(Debug)]
pub struct PlaybookSecondTurnNva {}

impl PlaybookSecondTurnNva {
    pub fn new() -> PlaybookSecondTurnNva {
        PlaybookSecondTurnNva {}
    }
}

impl Default for PlaybookSecondTurnNva {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for PlaybookSecondTurnNva {
    fn provide_command(&self, _active_card: u8, _map: &Map, _track: &Track) -> Vec<String> {
        // Decides to perform First Operation Only
        let mut vec = Vec::new();
        vec.push(String::from("op only").to_lowercase());
        vec.push(String::from("rally").to_lowercase());
        vec.push(String::from("north vietnam").to_lowercase());
        vec.push(String::from("the parrot's beak").to_lowercase());
        vec.push(String::from("kien phong").to_lowercase());
        vec.push(String::from("kien giang").to_lowercase());
        vec.push(String::from("stop").to_lowercase());

        // As part of rally they get the chance to improve the trail one level
        vec.push(String::from("yes").to_lowercase());

        vec
    }
}
