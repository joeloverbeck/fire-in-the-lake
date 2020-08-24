use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_87(
    _active_card: &Cards,
    _preview_card: &Cards,
    player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // Both ARVN and NVA have special instructions

    if faction == &Factions::NVA && player_types.get(&faction).unwrap() == &PlayerType::Ai {
        // Shaded: Replace any 2 ARVN with any 2 VC pieces within 2 spaces of Hue.
        // Patronage +4 or -4.
        // NVA special instructions: if VC a player, choose Op & Special Activity. If not, reduce Patronage.
        if player_types.get(&Factions::VC).unwrap() == &PlayerType::Human {
            return Ok(false);
        }

        // Effective at this point if Patronage > 0.
        return Ok(board.get_faction_stat(FactionStats::Patronage)? > 0);
    }

    panic!("Only implemented for NVA AI.");
}
