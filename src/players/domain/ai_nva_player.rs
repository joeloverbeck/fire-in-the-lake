use board::domain::board::Board;
use cards::domain::card::Cards;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::decision_making::whether_to_attack_or_ambush::whether_to_attack_or_ambush;
use players::domain::decision_making::whether_to_bombard::whether_to_bombard;
use players::domain::decision_making::whether_to_exploit_faction_capabilities_for_nva::whether_to_exploit_faction_capabilities_for_nva;
use players::domain::decision_making::whether_to_infiltrate::whether_to_infiltrate;
use players::domain::decision_making::whether_to_pass::whether_to_pass;
use players::domain::decision_making::whether_to_play_regular_event::whether_to_play_regular_event;
use players::domain::decision_making::whether_to_rally::whether_to_rally;
use players::domain::decision_making::whether_to_terror::whether_to_terror;
use players::domain::player::Player;
use players::domain::player_type::PlayerType;
use randomization::controllers::randomization_controller_trait::RandomizationControllers;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use sequence_of_play::domain::can_ai_faction_play_special_activity::can_ai_faction_play_special_activity;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use std::collections::HashMap;
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
        active_card: &Cards,
        preview_card: &Cards,
        _current_elegible_faction: Factions,
        player_types: HashMap<Factions, PlayerType>,
        possible_actions: Vec<SequenceOfPlaySlots>,
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

        possible_decision = whether_to_pass(Factions::NVA, board)?;

        if let Some(decision) = possible_decision {
            return Ok(decision);
        }

        possible_decision = whether_to_terror(
            Factions::NVA,
            board,
            &possible_actions,
            randomization_controller,
        )?;

        if let Some(decision) = possible_decision {
            // It has decided to terror. In this case NVA AI will also attempt to Bombard.
            if can_ai_faction_play_special_activity(&possible_actions)? {
                let possible_decision_to_bombard =
                    whether_to_bombard(Some(&decision), board, flags_controller)?;

                // If NVA bombarded, it must have added the main activity's mutations to it's decision, creating
                // a merged one.
                if let Some(decision_to_bombard) = possible_decision_to_bombard {
                    return Ok(decision_to_bombard);
                }
            }

            return Ok(decision);
        }

        possible_decision = whether_to_rally(
            Factions::NVA,
            &possible_actions,
            board,
            flags_controller,
            randomization_controller,
        )?;

        if let Some(decision) = possible_decision {
            // It has decided to Rally.

            // Additionally it will attempt to play the special activity *if it can*.
            if can_ai_faction_play_special_activity(&possible_actions)? {
                let possible_infiltrate_decision =
                    whether_to_infiltrate(Some(&decision), board, flags_controller)?;

                if let Some(infiltrate_decision) = possible_infiltrate_decision {
                    return Ok(infiltrate_decision);
                }
            }

            return Ok(decision);
        }

        panic!("NVA AI hasn't found any decision!");
    }

    fn get_player_type(&self) -> Result<PlayerType, String> {
        Ok(PlayerType::Ai)
    }
}

impl AiNvaPlayer {
    pub fn new() -> AiNvaPlayer {
        AiNvaPlayer {}
    }
}
