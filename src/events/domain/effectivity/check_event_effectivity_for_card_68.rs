use board::domain::board::Board;
use board::domain::queries::board_level_queries::are_there_any_us_irregulars_on_laos_or_cambodia::are_there_any_us_irregulars_on_laos_or_cambodia;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_68(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // Both ARVN and NVA have special instructions
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::NVA {
        // Shaded: Remove any 3 Irregulars to Available and set 1 of their Provinces to Active Opposition.
        // Special NVA instructions: if no Irregulars are on Laos or Cambodia, choose Op & Special Activity
        return Ok(are_there_any_us_irregulars_on_laos_or_cambodia(&board)?);
    }

    panic!("Card 68 only implemented for NVA AI.");
}
