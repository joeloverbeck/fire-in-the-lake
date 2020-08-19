use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::does_card_have_a_factions_capability::does_card_have_a_factions_capability;
use players::domain::does_card_have_a_factions_capability_and_of_a_specific_one::does_card_have_a_factions_capability_and_of_a_specific_one;
use players::domain::does_card_have_a_factions_capability_but_not_of_a_specific_one::does_card_have_a_factions_capability_but_not_of_a_specific_one;
use players::domain::events::is_current_non_capability_event_effective::is_current_non_capability_event_effective;
use players::domain::player::Player;
use players::domain::player_type::PlayerType;
use randomization::controllers::randomization_controller::RandomizationController;
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
        _possible_actions: Vec<String>,
        board: &Board,
        randomization_controller: &RandomizationController,
        _keyboard_input_controller: &KeyboardInputController,
        _display_controller: &DisplayController,
    ) -> Result<Decision, String> {
        // First off: Ambush and/or Troop Attack can remove Base or at least 1d6 enemies?
        if self
            .queries_controller
            .can_attack_remove_base(&Factions::NVA, board)?
            || self
                .queries_controller
                .can_attack_remove_a_number_of_enemies(
                    &Factions::NVA,
                    randomization_controller.roll_six_sided_die()?,
                    board,
                )?
        {
            panic!("NVA bot detected it can ambush or attack to remove base or 1d6 enemies.");
        } else if does_card_have_a_factions_capability_but_not_of_a_specific_one(
            &active_card,
            &Factions::NVA,
        )? && does_card_have_a_factions_capability_and_of_a_specific_one(
            &preview_card,
            &Factions::NVA,
        )? {
            // The next faction has NVA capabilities.

            if randomization_controller.roll_six_sided_die()?
                > board.get_number_of_arvn_leaders()?
            {
                panic!("Die throw was bigger than number of ARVN leaders in box. Not implemented.");
            } else {
                panic!("Not implemented.");
            }
        } else if does_card_have_a_factions_capability(&active_card)?
            && randomization_controller.roll_six_sided_die()?
                > board.get_number_of_arvn_leaders()?
        {
            // Play the capability
            panic!("The active card has any faction's capability and 1d6 is higher than the number of arvn leaders");
        } else if is_current_non_capability_event_effective(
            &active_card,
            &preview_card,
            player_types,
            &Factions::NVA,
            EventType::Shaded,
            board,
        )? {
            panic!("Was going to play the card for the event.");
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
