use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_96(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // It has the grey rifle, so AI ARVN will never play this event. VC and NVA have special instructions
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::ARVN {
        return Ok(false);
    }

    if (faction == &Factions::NVA) && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        // Shaded: If Tet Offensive played, return it to VC. If not, VC execute "General Uprising" as on the card (without using it).
        // NVA: If VC a player, choose Op & Special Activity. If Tet Offensive played, return it to VC. If not, Free Terror with 1 Underground VC per space.
        if player_types.get(&Factions::VC).unwrap() == &PlayerType::Human {
            return Ok(false);
        }

        // Whether or not Tet Offensive has been played, this should be effective.
        return Ok(true);
    }

    panic!("Not implemented for US nor VC");
}
