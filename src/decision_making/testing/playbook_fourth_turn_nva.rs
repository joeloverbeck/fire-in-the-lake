use board::map::Map;
use board::track::Track;
use decision_making::player::Player;

#[derive(Debug)]
pub struct PlaybookFourthTurnNva {}

impl PlaybookFourthTurnNva {
    pub fn new() -> PlaybookFourthTurnNva {
        PlaybookFourthTurnNva {}
    }
}

impl Default for PlaybookFourthTurnNva {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for PlaybookFourthTurnNva {
    fn provide_commands(&self, _active_card: u8, _map: &Map, _track: &Track) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.push(String::from("operation"));
        vec.push(String::from("march"));
        vec.push(String::from(
            "march_order:underground_nva_guerrillas:2:the parrot's beak:kien phong",
        ));
        vec.push(String::from(
            "march_order:underground_nva_guerrillas:2:the parrot's beak:kien giang",
        ));
        vec.push(String::from(
            "march_order:underground_nva_guerrillas:2:central laos:quang tri",
        ));
        vec.push(String::from(
            "march_order:underground_nva_guerrillas:5:north vietnam:quang tri",
        ));
        vec.push(String::from("infiltrate"));
        vec.push(String::from("infiltrate_instructions:place:southern laos"));
        vec.push(String::from(
            "infiltrate_instructions:exchange:southern laos",
        ));
        vec.push(String::from(
            "infiltrate_instructions:replace:southern laos:underground_vc_guerrilla",
        ));

        vec
    }
}
