use board::domain::board::Board;
use board::domain::queries::board_level_queries::are_there_any_casualties::are_there_any_casualties;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_120(
    _active_card: &Cards,
    _preview_card: &Cards,
    _player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // ARVN has special instructions.

    if faction == &Factions::NVA || faction == &Factions::VC {
        // Shaded: US Troop Casualties up to cards in RVN Leader box plus all US Base Casualties go out of play.
        // Effective if any forces in casualties.
        return Ok(are_there_any_casualties(board)?);
    }

    panic!("Implemented for NVA and VC.");
}
