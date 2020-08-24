use board::domain::board::Board;
use cards::domain::card::Cards;
use game_definitions::event_types::EventTypes;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::player_type::PlayerType;
use std::collections::HashMap;

pub fn check_event_effectivity_for_card_82(
    _active_card: &Cards,
    _preview_card: &Cards,
    _player_types: HashMap<Factions, PlayerType>,
    faction: &Factions,
    _preferible_event_type: EventTypes,
    board: &Board,
) -> Result<bool, String> {
    // ARVN and US have special instructions.

    // Shaded: 3 Available Us Troops out of play. Aid -9
    // This is effective if there are Available Us Troops or
    if faction == &Factions::NVA || faction == &Factions::VC {
        return Ok(board.get_forces_available(Forces::UsTroop)? > 0
            || board.get_faction_stat(FactionStats::Aid)? > 0);
    }

    panic!("Not implemented for ARVN and US");
}
