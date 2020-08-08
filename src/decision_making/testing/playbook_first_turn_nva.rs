use board::map::Map;
use board::track::Track;
use decision_making::player::Player;

#[derive(Debug)]
pub struct PlaybookFirstTurnNva {}

impl PlaybookFirstTurnNva {
    pub fn new() -> PlaybookFirstTurnNva {
        PlaybookFirstTurnNva {}
    }
}

impl Default for PlaybookFirstTurnNva {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for PlaybookFirstTurnNva {
    fn provide_command(&self, _active_card: u8, _map: &Map, _track: &Track) -> Vec<String> {
        // It should pass. Just return "pass".
        let mut vec = Vec::new();
        vec.push(String::from("pass").to_lowercase());

        vec
    }
}
