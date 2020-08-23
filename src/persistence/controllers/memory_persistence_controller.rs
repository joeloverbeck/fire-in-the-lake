use board::domain::board::Board;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::factions::Factions;
use persistence::domain::persist_faction_stats_mutations::persist_faction_stats_mutations;
use persistence::domain::persist_flags_mutations::persist_flags_mutations;
use persistence::domain::persist_forces_mutations::persist_forces_mutations;
use persistence::domain::persist_sequence_of_play_mutations::persist_sequence_of_play_mutations;
use persistence::domain::persist_space_mutations::persist_space_mutations;
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

        if decision.get_mutations()?.has_space_mutations()? {
            persist_space_mutations(decision.get_mutations()?.get_space_mutations()?, board)?;
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
    use board::domain::space::Space;
    use game_definitions::faction_stats::FactionStats;
    use game_definitions::forces::Forces;
    use game_definitions::space_identifiers::SpaceIdentifiers;
    use game_definitions::support_levels::SupportLevels;
    use players::domain::decision_information::DecisionInformation;
    use players::domain::faction_stats_mutation::FactionStatsMutation;
    use players::domain::forces_mutation::ForcesMutation;
    use players::domain::mutation_types::MutationTypes;
    use players::domain::mutations::Mutations;
    use players::domain::space_mutation::SpaceMutation;
    use players::domain::space_mutation_types::SpaceMutationTypes;

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

        let decision = Decision::new(mutations, Some(DecisionInformation::Event), None);

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

    #[test]
    fn test_after_persisting_a_terror_attack_the_values_have_changed_to_the_expected_ones(
    ) -> Result<(), String> {
        let mut board = Board::new();

        let sut = MemoryPersistenceController::new();

        let mut sequence_of_play_controller = SequenceOfPlayController::new();
        let mut flags_controller = FlagsController::new();

        board.set_faction_stat(FactionStats::NvaResources, 1)?;
        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            1,
            SpaceIdentifiers::PhuBonPhuYen,
        )?;
        board.set_support_level_of_space(
            SupportLevels::PassiveSupport,
            SpaceIdentifiers::PhuBonPhuYen,
        )?;

        let faction_stats_mutations: Vec<FactionStatsMutation> = vec![FactionStatsMutation::new(
            FactionStats::NvaResources,
            MutationTypes::Reduce,
            1,
            1,
        )];
        let forces_mutations: Vec<ForcesMutation> = vec![ForcesMutation::new(
            Forces::UndergroundNvaGuerrilla,
            MutationTypes::Flip,
            1,
            Some(SpaceIdentifiers::PhuBonPhuYen),
            Some(SpaceIdentifiers::PhuBonPhuYen),
        )];
        let space_mutations: Vec<SpaceMutation> = vec![
            SpaceMutation::new(
                SpaceIdentifiers::PhuBonPhuYen,
                SpaceMutationTypes::Terror,
                MutationTypes::Set,
                None,
            ),
            SpaceMutation::new(
                SpaceIdentifiers::PhuBonPhuYen,
                SpaceMutationTypes::Support,
                MutationTypes::ShiftDown,
                None,
            ),
        ];

        let mut mutations = Mutations::new();
        mutations.set_faction_stats_mutations(faction_stats_mutations)?;
        mutations.set_forces_mutations(forces_mutations)?;
        mutations.set_space_mutations(space_mutations)?;

        let decision = Decision::new(
            mutations,
            Some(DecisionInformation::Terror),
            Some(DecisionInformation::Bombard),
        );

        sut.persist_decision(
            &decision,
            &mut board,
            [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
            &mut sequence_of_play_controller,
            &mut flags_controller,
        )?;

        assert_eq!(board.get_faction_stat(FactionStats::NvaResources)?, 0);
        assert_eq!(
            board.get_forces_in_space(
                Forces::UndergroundNvaGuerrilla,
                SpaceIdentifiers::PhuBonPhuYen
            )?,
            0
        );
        assert_eq!(
            board
                .get_forces_in_space(Forces::ActiveNvaGuerrilla, SpaceIdentifiers::PhuBonPhuYen)?,
            1
        );
        assert_eq!(
            board.get_support_level_of_space(SpaceIdentifiers::PhuBonPhuYen)?,
            SupportLevels::Neutral
        );
        assert_eq!(
            board
                .get_space(SpaceIdentifiers::PhuBonPhuYen)?
                .has_terror()?,
            true
        );

        Ok(())
    }
}
