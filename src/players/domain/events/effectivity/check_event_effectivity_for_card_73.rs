use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_73(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // VC has special instructions, ARVN has a grey rifle
    if faction == &Factions::ARVN && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // Shaded: US moves 3 peices from available to out of play.
        // There must be some available.
        let queries_controller = QueriesController::new();

        return Ok(
            queries_controller.are_there_any_forces_of_a_faction_available(Factions::US, board)?
        );
    }

    panic!("Only implemented for ARVN and NVA.");
}
