use board::domain::board::Board;
use flags::controllers::flags_controller::FlagsController;
use players::domain::decision::Decision;
use players::domain::mutation_types::MutationTypes;
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
        sequence_of_play_controller: &mut SequenceOfPlayController,
        flags_controller: &mut FlagsController,
    ) -> Result<(), String> {
        // Goes through every mutation and manipulates either the board or the flags as necessary.
        for mutation in decision.get_sequence_of_play_mutations() {
            sequence_of_play_controller
                .register_pick(mutation.get_faction(), mutation.get_sequence_of_play_slot())?;
        }

        for mutation in decision.get_faction_stats_mutations() {
            if mutation.get_mutation_type() == &MutationTypes::Increase {
                board.increase_faction_stat(mutation.get_faction_stat(), mutation.get_value())?;
            } else if mutation.get_mutation_type() == &MutationTypes::Reduce {
                println!("Detected reduce mutation: {:?}", mutation);
                board.reduce_faction_stat(mutation.get_faction_stat(), mutation.get_value())?;
            } else {
                panic!("Case not handled for faction stats mutation {:?}", mutation);
            }
        }

        // Persist forces mutations
        for mutation in decision.get_forces_mutations() {
            if mutation.get_mutation_type() == &MutationTypes::Move {
                board.reduce_forces_in_space(
                    mutation.get_forces(),
                    mutation.get_from().unwrap(),
                    mutation.get_number(),
                )?;
                board.increase_forces_in_space(
                    mutation.get_forces(),
                    mutation.get_to().unwrap(),
                    mutation.get_number(),
                )?;
            } else {
                panic!(
                    "Case not handled for persist forces mutations type {:?}. Mutation: {:?}",
                    mutation.get_mutation_type(),
                    mutation
                );
            }
        }

        for mutation in decision.get_flags_mutations() {
            flags_controller.set_flag(*mutation.get_flag(), mutation.get_value())?;
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

        let decision = Decision::new(Vec::new(), faction_stats_mutations, Vec::new(), Vec::new());

        sut.persist_decision(
            &decision,
            &mut board,
            &mut sequence_of_play_controller,
            &mut flags_controller,
        )?;

        assert_eq!(board.get_faction_stat(FactionStats::NvaResources)?, 6);

        Ok(())
    }
}
