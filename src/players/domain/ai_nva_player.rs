use board::domain::board::Board;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::player::Player;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

#[derive(Debug)]
pub struct AiNvaPlayer {}

impl Default for AiNvaPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for AiNvaPlayer {
    fn decide(
        &self,
        _active_card: u8,
        _preview_card: u8,
        _current_elegible_faction: Factions,
        _possible_actions: Vec<String>,
        _board: &Board,
        _keyboard_input_controller: &KeyboardInputController,
        _display_controller: &DisplayController,
    ) -> Result<Decision, String> {
        todo!()
    }
}

impl AiNvaPlayer {
    pub fn new() -> AiNvaPlayer {
        AiNvaPlayer {}
    }
}
