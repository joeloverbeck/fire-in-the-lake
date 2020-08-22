use board::domain::board::Board;
use cards::domain::card::Cards;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::decision_production::produce_decision_to_pass::produce_decision_to_pass;
use players::domain::player::Player;
use players::domain::player_type::PlayerType;
use randomization::controllers::randomization_controller_trait::RandomizationControllers;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use std::collections::HashMap;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

#[derive(Debug)]
pub struct DummyPlayer {}

impl Default for DummyPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for DummyPlayer {
    fn decide(
        &self,
        _active_card: &Cards,
        _preview_card: &Cards,
        current_elegible_faction: Factions,
        _player_types: HashMap<Factions, PlayerType>,
        _possible_actions: Vec<SequenceOfPlaySlots>,
        board: &Board,
        _flags_controller: &FlagsController,
        _sequence_of_play_controller: &SequenceOfPlayController,
        _randomization_controller: &RandomizationControllers,
        _keyboard_input_controller: &KeyboardInputController,
        _display_controller: &DisplayController,
    ) -> Result<Decision, String> {
        // This dummy player just passes for every possible decision.

        Ok(produce_decision_to_pass(current_elegible_faction, board)?)
    }

    fn get_player_type(&self) -> Result<PlayerType, String> {
        Ok(PlayerType::Ai)
    }
}

impl DummyPlayer {
    pub fn new() -> DummyPlayer {
        DummyPlayer {}
    }
}
