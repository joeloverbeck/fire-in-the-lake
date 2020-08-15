use game_definitions::faction_stats::FactionStats;

use std::collections::HashMap;

#[derive(Clone)]
pub struct Board {
    faction_stats: HashMap<FactionStats, u8>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            faction_stats: [
                (FactionStats::Aid, 0),
                (FactionStats::TotalEcon, 0),
                (FactionStats::Patronage, 0),
                (FactionStats::VcResources, 0),
                (FactionStats::NvaResources, 0),
                (FactionStats::ArvnResources, 0),
                (FactionStats::SupportPlusAvailable, 0),
                (FactionStats::CoinPlusPatronage, 0),
                (FactionStats::OppositionPlusBases, 0),
                (FactionStats::NvaPlusBases, 0),
                (FactionStats::TheTrail, 0),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    pub fn set_faction_stat(
        &mut self,
        faction_stat: FactionStats,
        value: u8,
    ) -> Result<(), String> {
        *self.faction_stats.get_mut(&faction_stat).unwrap() = value;

        Ok(())
    }

    pub fn get_faction_stat(&self, faction_stat: FactionStats) -> Result<u8, String> {
        // Sanity checks
        if self.faction_stats.is_empty() {
            return Err("Attempted to get a faction stat from the board, but the container of faction stats was empty! The setup hasn't been done properly.".to_string());
        }

        if !self.faction_stats.contains_key(&faction_stat) {
            return Err(format!("Attempted to get the faction stat {:?}, but it couldn't be found on the board! Stored faction stats: {:?}", faction_stat, self.faction_stats));
        }

        let faction_stat_retrieval_result = self.faction_stats.get(&faction_stat);

        if let Some(value) = faction_stat_retrieval_result {
            Ok(*value)
        } else {
            Err(format!("Attempted to get the faction stat {:?} from the board, but it wasn't found there! Stored faction stats: {:?}", faction_stat, self.faction_stats))
        }
    }
}
