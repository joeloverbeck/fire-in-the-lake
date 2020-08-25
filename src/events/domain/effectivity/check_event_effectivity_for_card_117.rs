use board::domain::board::Board;
use board::domain::queries::board_level_queries::are_there_any_forces_of_a_faction_anywhere::are_there_any_forces_of_a_faction_anywhere;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_117(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // All but NVA have special instructions
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::NVA {
        // Shaded: Remove a die roll of ARVN pieces from 1 or 2 adjacent spaces. ARVN Ineligible through next card.
        // There must be ARVN pieces on the board.
        return Ok(are_there_any_forces_of_a_faction_anywhere(
            Factions::ARVN,
            board,
        )?);
    }

    panic!("Card 116 only implemented for NVA AI.");
}
