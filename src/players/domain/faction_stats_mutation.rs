use game_definitions::faction_stats::FactionStats;
use players::domain::mutation_types::MutationTypes;

#[derive(Debug)]
pub struct FactionStatsMutation {
    faction_stats: FactionStats,
    mutation_type: MutationTypes,
    value: u8,
}

impl FactionStatsMutation {
    pub fn new(
        faction_stats: FactionStats,
        mutation_type: MutationTypes,
        value: u8,
    ) -> FactionStatsMutation {
        FactionStatsMutation {
            faction_stats,
            mutation_type,
            value,
        }
    }
}
