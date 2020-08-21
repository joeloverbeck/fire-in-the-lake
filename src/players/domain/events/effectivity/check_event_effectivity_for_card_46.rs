use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_type::EventType;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_46(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventType,
    _board: &Board,
) -> Result<bool, String> {
    // US has grey rifle
    if faction == &Factions::US && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // Shaded: NVA free Infiltrate. Then NVA add 3 times and VC 2 times Trail value in Resources.
        // For now, this should be always valid. Maybe edge cases.
        return Ok(true);
    }

    panic!("Only implemented for US and NVA.");
}
