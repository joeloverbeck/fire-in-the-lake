use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_56(
    _active_card: &Cards,
    _preview_card: &Cards,
    _player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // All have special instructions.

    if faction == &Factions::NVA {
        // The special instructions refer to what operations and special activities to perform.
        return Ok(true);
    }

    panic!("Only implemented for NVA.");
}
