use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_10(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    _board: &Board,
) -> Result<bool, String> {
    // VC has grey rifle.
    if faction == &Factions::VC && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // Shaded: -5 ARVN Resources. No Air Strike until Coup. MOMENTUM.
        // This should always be effective.
        return Ok(true);
    }

    panic!("Only implemented for VC and NVA");
}
