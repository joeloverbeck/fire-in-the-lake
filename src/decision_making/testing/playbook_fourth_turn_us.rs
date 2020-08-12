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
        vec.push(String::from("quang tri"));

        // This one is tricky and the first time I have to do this. The player would never
        // enter this; it should be a result of the announcer asking very specific questions
        // and then programatically that HumanUS player struct creating this very
        // specific string to send as special instructions for the system as part of the event
        // in play.
        vec.push(String::from(
            "event_instruction deploy_from_out_of_play us_troops:2:saigon",
        ));
        vec.push(String::from(
            "event_instruction deploy_from_out_of_play us_troops:3:hue",
        ));
        vec.push(String::from(
            "event_instruction deploy_from_out_of_play us_base:1:hue",
        ));

        vec
    }
}
