use board::map::Map;
use board::track::Track;
use decision_making::player::Player;

#[derive(Debug)]
pub struct PlaybookFirstTurnVc {}

impl PlaybookFirstTurnVc {
    pub fn new() -> PlaybookFirstTurnVc {
        PlaybookFirstTurnVc {}
    }
}

impl Default for PlaybookFirstTurnVc {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for PlaybookFirstTurnVc {
    fn provide_command(&self, _active_card: u8, _map: &Map, _track: &Track) -> Vec<String> {
        // During the first turn it just decides to execute the event. He just should have to say 'event'.
        // It's the receiver that should take into consideration which event each faction would trigger (want to).
        let mut vec = Vec::new();
        vec.push(String::from("event").to_lowercase());

        vec
    }
}
