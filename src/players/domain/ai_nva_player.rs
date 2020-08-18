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
        active_card: &RegularCard,
        preview_card: &RegularCard,
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
            if (active_card.has_any_faction_capability()?
                && !(active_card.get_faction_capability()? == Factions::NVA))
                && (preview_card.has_any_faction_capability()?
                    && preview_card.get_faction_capability()? == Factions::NVA)
            {
                // The next faction has NVA capabilities.

                if randomization_controller.roll_six_sided_die()?
                    > board.get_number_of_arvn_leaders()?
                {
                    panic!(
                        "Die throw was bigger than number of ARVN leaders in box. Not implemented."
                    );
                }
                else{
                    panic!("Not implemented.");
                }
            }
        }

        todo!()
    }
}

impl AiNvaPlayer {
    pub fn new() -> AiNvaPlayer {
        AiNvaPlayer {
            queries_controller: QueriesController::new(),
        }
    }
}
