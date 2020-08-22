use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_58(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // VC and US have grey rifles. NVA has special instructions.

    if (faction == &Factions::VC || faction == &Factions::US)
        && player_types.get(&faction).unwrap() == &PlayerType::Ai
    {
        return Ok(false);
    }

    if faction == &Factions::NVA && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        // Shaded: If no COIN cubes in Laos, Improve Trail 2 boxes. If there are, US and ARVN Redeploy them to Vietnam.
        // Always effective.

        return Ok(true);
    }

    panic!("Not implemented for ARVN");
}
