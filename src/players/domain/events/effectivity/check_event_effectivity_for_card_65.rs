use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_65(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventType,
    board: &Board,
) -> Result<bool, String> {
    // Has a grey rifle for ARVN
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::ARVN {
        return Ok(false);
    }

    let queries_controller = QueriesController::new();

    if faction == &Factions::NVA || faction == &Factions::VC {
        // Shaded: US must remove a die roll in pieces from the map from
        // the map to out of play.
        // So a the entire faction has to have any piece in the map.
        return Ok(
            queries_controller.are_there_any_forces_of_a_faction_anywhere(Factions::US, &board)?
        );
    }

    panic!("Card 65 not implemented for US.");
}
