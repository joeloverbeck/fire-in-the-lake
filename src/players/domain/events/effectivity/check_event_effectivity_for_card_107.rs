use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_107(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // US has grey rifle. NVA has special instructions.
    if player_types.get(faction).unwrap() == &PlayerType::Ai && faction == &Factions::US {
        return Ok(false);
    }

    if faction == &Factions::NVA && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        // Shaded: Shift Saigon 1 level toward Active Opposition. Aid -12.
        // NVA: If VC a player, choose Op & Special Activity.
        if player_types.get(&Factions::VC).unwrap() == &PlayerType::Human {
            return Ok(false);
        }

        // There are edge cases, but won't program their effectivity in advance.
        return Ok(true);
    }

    panic!("Only implemented for US and NVA AIs");
}
