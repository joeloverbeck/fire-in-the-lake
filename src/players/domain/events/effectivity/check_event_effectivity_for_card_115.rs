use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_115(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventType,
    _board: &Board,
) -> Result<bool, String> {
    if player_types.get(faction).unwrap() == &PlayerType::Ai
        && (faction == &Factions::NVA || faction == &Factions::ARVN || faction == &Factions::US)
    {
        return Ok(false);
    }

    panic!("Card 115 only implemented for NVA, US and ARVN AIs.");
}
