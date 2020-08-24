use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_2(
    _active_card: &Cards,
    _preview_card: &Cards,
    _player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // All factions but NVA have special instructions

    // Shaded: NVA places 2 pieces in Cambodia. US moves any 2 US Troops to out of play. Aid -6.
    if faction == &Factions::NVA {
        // This is always effective.
        return Ok(true);
    }

    panic!("Not implemented for US, ARVN and VC");
}
