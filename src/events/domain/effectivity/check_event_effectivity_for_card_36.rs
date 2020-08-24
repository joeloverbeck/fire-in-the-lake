use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_36(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // None have grey rifles.

    // Shaded:
    // Place a Tunnel on an NVA or VC Highland Base. 3 US Troops there to Casualties.
    // NVA & VC special instructions: Remove as many US Troops as possible, within
    // that, place a Tunnel of own faction if possible
    if faction == &Factions::NVA && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        todo!()
    }

    panic!("Only implemented for NVA AIs");
}
