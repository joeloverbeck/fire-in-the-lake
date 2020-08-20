use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_53(
    _active_card: &Cards,
    _preview_card: &Cards,
    _player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventType,
    board: &Board,
) -> Result<bool, String> {
    if faction == &Factions::NVA || faction == &Factions::VC {
        // Shaded: Remove up to 1 US and 2 ARVN Bases from any provinces (US to Casualties).
        // So there must be at least 1 us base or at least 1 ARVN base anywhere.
        let queries_controller = QueriesController::new();

        return Ok(queries_controller
            .is_there_a_specific_force_anywhere(Forces::UsBase, &board)?
            || queries_controller.is_there_a_specific_force_anywhere(Forces::ArvnBase, &board)?);
    }

    panic!("Card 53 only implemented for NVA and VC AIs.");
}
