use players::domain::faction_stats_mutation::FactionStatsMutation;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;

#[derive(Debug)]
pub struct Decision {
    sequence_of_play_mutations: Vec<SequenceOfPlayMutation>,
    faction_stats_mutations: Vec<FactionStatsMutation>,
}

impl Decision {
    pub fn new(
        sequence_of_play_mutations: Vec<SequenceOfPlayMutation>,
        faction_stats_mutations: Vec<FactionStatsMutation>,
    ) -> Decision {
        Decision {
            sequence_of_play_mutations,
            faction_stats_mutations,
        }
    }

    pub fn get_sequence_of_play_mutations(&self) -> &Vec<SequenceOfPlayMutation> {
        &self.sequence_of_play_mutations
    }

    pub fn get_faction_stats_mutations(&self) -> &Vec<FactionStatsMutation> {
        &self.faction_stats_mutations
    }
}
