use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_62(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // It has the grey rifle, so AI VC will never play this event. US has special instructions.
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::VC {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // Shaded: NVA places a total of 12 NVA Troops and Guerrillas in Cambodia.
        // This should always be effective.
        return Ok(true);
    }

    panic!("Card 62 only implemented for NVA and VC AIs.");
}
