use board::domain::board::Board;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::dummy_player::DummyPlayer;
use players::domain::human_us_player::HumanUsPlayer;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

extern crate enum_dispatch;
use self::enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Player {
    fn decide(
        &self,
        active_card: u8,
        preview_card: u8,
        current_elegible_faction: Factions,
        possible_actions: Vec<String>,
        board: &Board,
        keyboard_input_controller: &KeyboardInputController,
        display_controller: &DisplayController,
    ) -> Result<Decision, String>;
}

#[enum_dispatch(Player)]
#[derive(Debug)]
pub enum Players {
    HumanUsPlayer,
    DummyPlayer,
}
