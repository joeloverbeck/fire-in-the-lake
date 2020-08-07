extern crate enum_dispatch;

mod board;
pub mod cards;
mod commands;
pub mod decision_making;
pub mod factions;
pub mod game_flow;
mod track;

struct NvaStats {
    victory_level: u8,
}

impl NvaStats {
    fn new() -> NvaStats {
        NvaStats { victory_level: 0 }
    }

    fn get_victory_level(&self) -> u8 {
        self.victory_level
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_on_creation_nva_victory_level_should_be_zero() {
        let nva_stats = NvaStats::new();

        assert_eq!(nva_stats.get_victory_level(), 0);
    }
}
