use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_78(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    if player_types.get(faction).unwrap() == &PlayerType::Ai
        && (faction == &Factions::NVA || faction == &Factions::VC)
    {
        return Ok(false);
    }

    panic!("Card 115 only implemented for NVA and VC AIs.");
}
