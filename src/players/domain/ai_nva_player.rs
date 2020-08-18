use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::regular_card::RegularCard;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::player::Player;
use randomization::controllers::randomization_controller::RandomizationController;
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
        _active_card: &RegularCard,
        _preview_card: &RegularCard,
        _current_elegible_faction: Factions,
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
        } else {
            panic!("NVA bot detected it cannot ambush or attack to remove base or 1d6 enemies.");

            // Is the next card NVA Capability but the current card is not?
        }
    }
}

impl AiNvaPlayer {
    pub fn new() -> AiNvaPlayer {
        AiNvaPlayer {
            queries_controller: QueriesController::new(),
        }
    }
}
