use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_26(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // Special instructions for NVA and US

    // Shaded: 3 Irregulars map to Casualties. Shift each space they were in 1 level toward Active Opposition.
    if faction == &Factions::NVA && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        // NVA Ai is asking. Special instructions.
        // If no irregulars are in Laos or Cambodia, choose Op & Special Activity instead.
        let queries_controller = QueriesController::new();

        return Ok(queries_controller.are_there_any_us_irregulars_on_laos_or_cambodia(&board)?);
    };

    todo!()
}
