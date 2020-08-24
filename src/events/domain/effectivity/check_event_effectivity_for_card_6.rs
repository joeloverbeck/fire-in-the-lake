use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_6(
    _active_card: &Cards,
    _preview_card: &Cards,
    _player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // US, VC and ARVN have special instructions.

    // Shaded: 2 Available US Troops to Casualties. Improve Trail by 2 boxes.
    if faction == &Factions::NVA {
        // This should always be effective.
        return Ok(true);
    }

    panic!("Not implemented for US, ARVN or VC.");
}
