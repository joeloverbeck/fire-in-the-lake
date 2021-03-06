use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_41(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    if player_types.get(faction).unwrap() == &PlayerType::Ai {
        // It has grey rifles for all the factions. None of them will play it if they are AI.
        return Ok(false);
    }

    panic!("Human event effectivity not implemented for card 41");
}
