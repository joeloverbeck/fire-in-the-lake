use board::map::Map;
use board::track::Track;
use decision_making::player::Player;

#[derive(Debug)]
pub struct PlaybookFourthTurnUs {}

impl PlaybookFourthTurnUs {
    pub fn new() -> PlaybookFourthTurnUs {
        PlaybookFourthTurnUs {}
    }
}

impl Default for PlaybookFourthTurnUs {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for PlaybookFourthTurnUs {
    fn provide_commands(&self, _active_card: u8, _map: &Map, _track: &Track) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.push(String::from("event"));

        vec
    }
}
