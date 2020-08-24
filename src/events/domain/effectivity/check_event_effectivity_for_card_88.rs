use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use game_definitions::space_identifiers::SpaceIdentifiers;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_88(
    _active_card: &Cards,
    _preview_card: &Cards,
    _player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // VC and US have special instructions

    // Shaded: Shift Saigon 1 level toward Neutral. Patronage -5. ARVN Ineligible through next card.
    if faction == &Factions::NVA {
        // Only if Saigon has Support or Patronage > 0.
        let queries_controller = QueriesController::new();

        return Ok(board.get_faction_stat(FactionStats::Patronage)? > 0
            || queries_controller
                .does_space_identifier_have_support(SpaceIdentifiers::Saigon, board)?);
    }

    panic!("Only implemented for NVA");
}
