use board::map::Map;
use board::track::Track;

extern crate enum_dispatch;
use self::enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Player {
    fn provide_command(&self, active_card: u8, map: &Map, track: &Track) -> Vec<String>;
}

#[enum_dispatch(Player)]
#[derive(Debug)]
pub enum Players {
    PlaybookFirstTurnVc,
    PlaybookFirstTurnNva,
}

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
