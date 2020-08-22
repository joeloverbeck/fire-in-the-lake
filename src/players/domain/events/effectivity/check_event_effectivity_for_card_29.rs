use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_29(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // US, ARVN and NVA have special instructions.

    if faction == &Factions::NVA && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        // Shaded: Replace all Irregulars with VC Guerrillas. 1 Neutral Highland to Active Opposition. -3 Patronage.
        // NVA: If no Irregulars are in Laos or Cambodia, choose Op & Special Activity instead.
        let queries_controller = QueriesController::new();

        if !queries_controller.are_there_any_us_irregulars_on_laos_or_cambodia(&board)? {
            return Ok(false);
        }

        // This is virtually always effective.
        return Ok(true);
    }

    panic!("Only implemented for NVA");
}
