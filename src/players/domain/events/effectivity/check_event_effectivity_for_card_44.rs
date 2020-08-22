use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_44(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // VC has a grey rifle. US has special instructions.
    if faction == &Factions::VC && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // Shaded: Select a Province with NVA Troops then remove a die roll of US Troops within 1 space of it to casualties.
        // There must be NVA troops somewhere.
        let queries_controller = QueriesController::new();

        return Ok(queries_controller.is_there_a_specific_force_anywhere(Forces::NvaTroop, board)?);
    }

    panic!("Only implemented for VC and NVA.");
}
