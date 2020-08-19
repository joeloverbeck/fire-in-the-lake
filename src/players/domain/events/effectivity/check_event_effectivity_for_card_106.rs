use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_106(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventType,
    _board: &Board,
) -> Result<bool, String> {
    // It has the grey rifle, so AI NVA will never play this event.
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::NVA {
        return Ok(false);
    }

    panic!("Card 106 only implemented for NVA AI.");
}
