use board::domain::board::Board;
use cards::domain::card::Cards;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::factions::Factions;
use players::domain::ai_nva_player::AiNvaPlayer;
use players::domain::decision::Decision;
use players::domain::dummy_player::DummyPlayer;
use players::domain::human_us_player::HumanUsPlayer;
use players::domain::player_type::PlayerType;
use randomization::controllers::randomization_controller_trait::RandomizationControllers;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use std::collections::HashMap;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

extern crate enum_dispatch;
use self::enum_dispatch::enum_dispatch;

#[enum_dispatch]
#[allow(clippy::too_many_arguments)]
pub trait Player {
    fn decide(
        &self,
        active_card: &Cards,
        preview_card: &Cards,
        current_elegible_faction: Factions,
        player_types: HashMap<Factions, PlayerType>,
        possible_actions: Vec<SequenceOfPlaySlots>,
        board: &Board,
        flags_controller: &FlagsController,
        sequence_of_play_controller: &SequenceOfPlayController,
        randomization_controller: &RandomizationControllers,
        keyboard_input_controller: &KeyboardInputController,
        display_controller: &DisplayController,
    ) -> Result<Decision, String>;
    fn get_player_type(&self) -> Result<PlayerType, String>;
}

#[enum_dispatch(Player)]
#[derive(Debug)]
pub enum Players {
    AiNvaPlayer,
    HumanUsPlayer,
    DummyPlayer,
}
