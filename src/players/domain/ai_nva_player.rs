use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::decision_making::whether_to_attack_or_ambush::whether_to_attack_or_ambush;
use players::domain::decision_making::whether_to_exploit_faction_capabilities_for_nva::whether_to_exploit_faction_capabilities_for_nva;
use players::domain::decision_making::whether_to_play_regular_event::whether_to_play_regular_event;
use players::domain::player::Player;
use players::domain::player_type::PlayerType;
use randomization::controllers::randomization_controller_trait::RandomizationControllers;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use std::collections::HashMap;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

#[derive(Debug)]
pub struct AiNvaPlayer {
    queries_controller: QueriesController,
}

impl Default for AiNvaPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for AiNvaPlayer {
    fn decide(
        &self,
        active_card: &Cards,
        preview_card: &Cards,
        _current_elegible_faction: Factions,
        player_types: HashMap<Factions, PlayerType>,
        _possible_actions: Vec<SequenceOfPlaySlots>,
        board: &Board,
        flags_controller: &FlagsController,
        sequence_of_play_controller: &SequenceOfPlayController,
        randomization_controller: &RandomizationControllers,
        _keyboard_input_controller: &KeyboardInputController,
        _display_controller: &DisplayController,
    ) -> Result<Decision, String> {
        // First off: Ambush and/or Troop Attack can remove Base or at least 1d6 enemies?

        let mut possible_decision: Option<Decision>;

        possible_decision = whether_to_attack_or_ambush(board, randomization_controller)?;

        if let Some(decision) = possible_decision {
            return Ok(decision);
        }

        possible_decision = whether_to_exploit_faction_capabilities_for_nva(
            active_card,
            preview_card,
            board,
            randomization_controller,
        )?;

        if let Some(decision) = possible_decision {
            return Ok(decision);
        }

        possible_decision = whether_to_play_regular_event(
            &Factions::NVA,
            EventTypes::Shaded,
            active_card,
            preview_card,
            player_types,
            board,
            flags_controller,
            sequence_of_play_controller,
        )?;

        if let Some(decision) = possible_decision {
            return Ok(decision);
        }

        todo!()
    }

    fn get_player_type(&self) -> Result<PlayerType, String> {
        Ok(PlayerType::Ai)
    }
}

impl AiNvaPlayer {
    pub fn new() -> AiNvaPlayer {
        AiNvaPlayer {
            queries_controller: QueriesController::new(),
        }
    }
}
