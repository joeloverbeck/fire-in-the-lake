use board::map::Map;
use board::track::Track;
use decision_making::player::Player;

#[derive(Debug)]
pub struct PlaybookThirdTurnArvn {}

impl PlaybookThirdTurnArvn {
    pub fn new() -> PlaybookThirdTurnArvn {
        PlaybookThirdTurnArvn {}
    }
}

impl Default for PlaybookThirdTurnArvn {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for PlaybookThirdTurnArvn {
    fn provide_commands(&self, _active_card: u8, _map: &Map, _track: &Track) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.push(String::from("event").to_lowercase());
        vec.push(String::from("binh dinh").to_lowercase());

        vec
    }
}
