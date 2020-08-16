use game_definitions::faction_stats::FactionStats;
use players::domain::mutation_types::MutationTypes;

#[derive(Debug)]
pub struct FactionStatsMutation {
    faction_stat: FactionStats,
    mutation_type: MutationTypes,
    previous_value: u8,
    value: u8,
}

impl FactionStatsMutation {
    pub fn new(
        faction_stat: FactionStats,
        mutation_type: MutationTypes,
        previous_value: u8,
        value: u8,
    ) -> FactionStatsMutation {
        FactionStatsMutation {
            faction_stat,
            mutation_type,
            previous_value,
            value,
        }
    }

    pub fn get_faction_stat(&self) -> &FactionStats {
        &self.faction_stat
    }

    pub fn get_mutation_type(&self) -> &MutationTypes {
        &self.mutation_type
    }

    pub fn get_previous_value(&self) -> u8 {
        self.previous_value
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
}
