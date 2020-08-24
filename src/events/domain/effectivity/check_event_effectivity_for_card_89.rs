use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_89(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // US has grey rifle. VC has special instructions.
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::US {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // Shaded: Place a VC piece in and shift Saigon 1 level toward Passive Opposition. Patronage -6.
        // This is always effective.
        return Ok(true);
    }

    panic!("Only implemented for US and NVA AIs");
}
