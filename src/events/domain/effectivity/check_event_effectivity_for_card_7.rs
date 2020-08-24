use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_7(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // VC has grey rifle.
    if faction == &Factions::VC && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        return Ok(false);
    }

    if faction == &Factions::NVA {
        // Shaded: Improve Trail by 1 box and to a minimum of 2. ARVN Resources -9.
        // This is virtually always effective, but will make minimum check.
        return Ok(board.get_faction_stat(FactionStats::ArvnResources)? > 0);
    }

    panic!("Only implemented for VC and NVA");
}
