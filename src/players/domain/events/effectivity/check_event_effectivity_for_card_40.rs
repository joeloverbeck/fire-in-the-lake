use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_40(
    _active_card: &Cards,
    _preview_card: &Cards,
    _player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // VC and ARVN have special instructions.

    if faction == &Factions::NVA {
        // Shaded: 3 US Troops from Available to Casualties.
        return Ok(board.get_forces_available(Forces::UsTroop)? > 0);
    }

    panic!("Only implemented for NVA.");
}
