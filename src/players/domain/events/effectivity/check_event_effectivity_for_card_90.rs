use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_90(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // US has grey rifle, VC has special instructions.
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::US {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // Shaded: Place any 1 Guerrilla in each Province with ARVN. ARVN Troops Redeploy as if no Bases.
        // This is virtually always effective, but check if there are ARVN forces anywhere.
        let sequence_of_play_controller = QueriesController::new();

        return Ok(sequence_of_play_controller
            .are_there_any_forces_of_a_faction_anywhere(Factions::ARVN, board)?);
    }

    panic!("Only implemented for US AI and NVA.");
}
