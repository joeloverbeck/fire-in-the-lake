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
    fn provide_command(&self, _active_card: u8, _map: &Map, _track: &Track) -> Vec<String> {
        // It will do lotsa stuff.
        // operation
        // train saigon
        // 6 (as in six troops from his available)
        // yes (as in you want pacify, which is allowed because of train or something)
        // pacify saigon
        // yes (as in you want to perform a special activity)
        // govern
        // an loc
        // can tho
        // stop

        let mut vec = Vec::new();
        vec.push(String::from("operation").to_lowercase());
        vec.push(String::from("train saigon").to_lowercase());
        vec.push(String::from("6").to_lowercase());
        vec.push(String::from("yes").to_lowercase());
        vec.push(String::from("pacify saigon").to_lowercase());
        vec.push(String::from("yes").to_lowercase());
        vec.push(String::from("govern").to_lowercase());
        vec.push(String::from("an loc").to_lowercase());
        vec.push(String::from("can tho").to_lowercase());
        vec.push(String::from("stop").to_lowercase());

        vec
    }
}
