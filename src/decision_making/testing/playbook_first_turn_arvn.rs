use board::map::Map;
use board::track::Track;
use decision_making::player::Player;

#[derive(Debug)]
pub struct PlaybookFirstTurnArvn {}

impl PlaybookFirstTurnArvn {
    pub fn new() -> PlaybookFirstTurnArvn {
        PlaybookFirstTurnArvn {}
    }
}

impl Default for PlaybookFirstTurnArvn {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for PlaybookFirstTurnArvn {
    fn provide_commands(&self, _active_card: u8, _map: &Map, _track: &Track) -> Vec<String> {
        // It will do lotsa stuff.
        // operation
        // train saigon
        // 6 (as in six troops from his available)
        // pacify
        // govern
        // an loc
        // can tho

        let mut vec = Vec::new();
        vec.push(String::from("operation").to_lowercase());
        vec.push(String::from("train").to_lowercase());
        vec.push(String::from("saigon").to_lowercase());
        vec.push(String::from("6").to_lowercase());
        vec.push(String::from("pacify").to_lowercase());
        vec.push(String::from("govern").to_lowercase());
        vec.push(String::from("an loc").to_lowercase());
        vec.push(String::from("can tho").to_lowercase());

        vec
    }
}
