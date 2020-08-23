use board::domain::board::Board;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::factions::Factions;
use persistence::domain::persist_faction_stats_mutations::persist_faction_stats_mutations;
use persistence::domain::persist_flags_mutations::persist_flags_mutations;
use persistence::domain::persist_forces_mutations::persist_forces_mutations;
use persistence::domain::persist_sequence_of_play_mutations::persist_sequence_of_play_mutations;
use players::domain::decision::Decision;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;

pub struct MemoryPersistenceController {}

impl Default for MemoryPersistenceController {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryPersistenceController {
    pub fn new() -> MemoryPersistenceController {
        MemoryPersistenceController {}
    }

    pub fn persist_decision(
        &self,
        decision: &Decision,
        board: &mut Board,
        faction_order: [Factions; 4],
        sequence_of_play_controller: &mut SequenceOfPlayController,
        flags_controller: &mut FlagsController,
    ) -> Result<(), String> {
        // Goes through every mutation and manipulates either the board or the flags as necessary.
        if decision.get_mutations()?.has_sequence_of_play_mutations()? {
            persist_sequence_of_play_mutations(
                decision.get_mutations()?.get_sequence_of_play_mutations()?,
                faction_order,
                sequence_of_play_controller,
            )?;
        }

        if decision.get_mutations()?.has_faction_stats_mutations()? {
            persist_faction_stats_mutations(
                decision.get_mutations()?.get_faction_stats_mutations()?,
                board,
            )?;
        }

        // Persist forces mutations
        if decision.get_mutations()?.has_forces_mutations()? {
            persist_forces_mutations(decision.get_mutations()?.get_forces_mutations()?, board)?;
        }

        if decision.get_mutations()?.has_flags_mutations()? {
            persist_flags_mutations(
                decision.get_mutations()?.get_flags_mutations()?,
                flags_controller,
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use board::controllers::setup_controller::SetupController;
    use game_definitions::faction_stats::FactionStats;
    use players::domain::faction_stats_mutation::FactionStatsMutation;
    use players::domain::mutation_types::MutationTypes;
    use players::domain::mutations::Mutations;

    #[test]
    fn test_after_persisting_a_faction_stat_mutation_the_value_has_changed_to_the_expected_value(
    ) -> Result<(), String> {
        let (mut board, _) = SetupController::new().setup_full().unwrap();

        let sut = MemoryPersistenceController::new();

        let mut sequence_of_play_controller = SequenceOfPlayController::new();
        let mut flags_controller = FlagsController::new();

        let mut faction_stats_mutations: Vec<FactionStatsMutation> = Vec::new();
        faction_stats_mutations.push(FactionStatsMutation::new(
            FactionStats::NvaResources,
            MutationTypes::Reduce,
            board.get_faction_stat(FactionStats::NvaResources)?,
            4,
        ));

        let mut mutations = Mutations::new();

        mutations.set_faction_stats_mutations(faction_stats_mutations)?;

        let decision = Decision::new(mutations);

        sut.persist_decision(
            &decision,
            &mut board,
            [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
            &mut sequence_of_play_controller,
            &mut flags_controller,
        )?;

        assert_eq!(board.get_faction_stat(FactionStats::NvaResources)?, 6);

        Ok(())
    }
}
