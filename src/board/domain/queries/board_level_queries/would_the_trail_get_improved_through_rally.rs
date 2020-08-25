use game_definitions::faction_stats::FactionStats;

use board::domain::board::Board;

pub fn would_the_trail_get_improved_through_rally(board: &Board) -> Result<bool, String> {
    // It would get improved if NVA >= 3 resources if The Trail < 9 || The Trail >= 3 and NVA >= 9
    Ok((board.get_faction_stat(FactionStats::TheTrail)? < 9
        && board.get_faction_stat(FactionStats::NvaResources)? >= 3)
        || (board.get_faction_stat(FactionStats::TheTrail)? >= 3
            && board.get_faction_stat(FactionStats::NvaResources)? >= 9))
}
