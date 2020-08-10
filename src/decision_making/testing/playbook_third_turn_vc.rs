use board::map::Map;
use board::track::Track;
use decision_making::player::Player;

#[derive(Debug)]
pub struct PlaybookThirdTurnVc {}

impl PlaybookThirdTurnVc {
    pub fn new() -> PlaybookThirdTurnVc {
        PlaybookThirdTurnVc {}
    }
}

impl Default for PlaybookThirdTurnVc {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for PlaybookThirdTurnVc {
    fn provide_commands(&self, _active_card: u8, _map: &Map, _track: &Track) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.push(String::from("operation"));
        vec.push(String::from("rally"));
        vec.push(String::from("pleiku"));
        vec.push(String::from("quang tri"));
        vec.push(String::from("hue"));
        vec.push(String::from("stop"));
        vec.push(String::from("yes"));
        vec.push(String::from("tax"));
        vec.push(String::from("quang tin"));
        vec.push(String::from("quang duc"));
        vec.push(String::from("binh tuy"));
        vec.push(String::from("stop"));

        vec
    }
}
