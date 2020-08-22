use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_50(
    _active_card: &Cards,
    _preview_card: &Cards,
    _player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // ARVN and US have special instructions

    // Shaded: VC then NVA each execute any 3 free Limited Operations.
    if faction == &Factions::NVA || faction == &Factions::VC {
        // This is always effective
        return Ok(true);
    }

    panic!("Only implemented for NVA and VC");
}
