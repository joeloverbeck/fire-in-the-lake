use board::domain::board::Board;
use game_definitions::faction_stats::FactionStats;

pub struct SetupController {}

impl Default for SetupController {
    fn default() -> Self {
        Self::new()
    }
}

impl SetupController {
    pub fn new() -> SetupController {
        SetupController {}
    }

    pub fn setup_full(&self) -> Result<(Board, Vec<String>), String> {
        let mut instructions: Vec<String> = Vec::new();

        let mut board = Board::new();

        // Set the full scenario's faction stats
        board.set_faction_stat(FactionStats::Aid, 15)?;
        board.set_faction_stat(FactionStats::TotalEcon, 15)?;
        board.set_faction_stat(FactionStats::Patronage, 15)?;
        board.set_faction_stat(FactionStats::VcResources, 5)?;
        board.set_faction_stat(FactionStats::NvaResources, 10)?;
        board.set_faction_stat(FactionStats::ArvnResources, 30)?;
        board.set_faction_stat(FactionStats::SupportPlusAvailable, 38)?;
        board.set_faction_stat(FactionStats::CoinPlusPatronage, 35)?;
        board.set_faction_stat(FactionStats::OppositionPlusBases, 27)?;
        board.set_faction_stat(FactionStats::NvaPlusBases, 4)?;
        board.set_faction_stat(FactionStats::TheTrail, 1)?;

        // Push the instructions to the player in order to put those markers
        instructions.push(format!("Set {} to 15.", FactionStats::Aid));
        instructions.push(format!("Set {} to 15.", FactionStats::TotalEcon));
        instructions.push(format!("Set {} to 15.", FactionStats::Patronage));
        instructions.push(format!("Set {} to 5.", FactionStats::VcResources));
        instructions.push(format!("Set {} to 10.", FactionStats::NvaResources));
        instructions.push(format!("Set {} to 30.", FactionStats::ArvnResources));
        instructions.push(format!("Set {} to 38.", FactionStats::SupportPlusAvailable));
        instructions.push(format!("Set {} to 35.", FactionStats::CoinPlusPatronage));
        instructions.push(format!("Set {} to 27.", FactionStats::OppositionPlusBases));
        instructions.push(format!("Set {} to 4.", FactionStats::NvaPlusBases));
        instructions.push(format!("Set {} to 1.", FactionStats::TheTrail));

        Ok((board, instructions))
    }
}
