use board::map::Map;
use board::track::Track;
use decision_making::player::Player;

#[derive(Debug)]
pub struct PlaybookSecondTurnUs {}

impl PlaybookSecondTurnUs {
    pub fn new() -> PlaybookSecondTurnUs {
        PlaybookSecondTurnUs {}
    }
}

impl Default for PlaybookSecondTurnUs {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for PlaybookSecondTurnUs {
    fn provide_command(&self, _active_card: u8, _map: &Map, _track: &Track) -> Vec<String> {
        // Decides to perform Second Limited Operation.
        let mut vec = Vec::new();
        vec.push(String::from("sweep").to_lowercase());
        vec.push(String::from("quang tri").to_lowercase());

        vec
    }
}
