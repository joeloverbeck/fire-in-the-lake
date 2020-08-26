use game_definitions::constants::MAXIMUM_NUMBER_OF_NVA_UNITS_ANY_BASE_MUST_BE_PROTECTED_BY_UNDER_WHICH_NVA_WILL_ATTEMPT_TO_RALLY;
use game_definitions::constants::MAXIMUM_TRAIL_NUMBER_UNDER_WHICH_NVA_WILL_ATTEMPT_TO_RALLY;
use board::domain::queries::board_level_queries::can_nva_place_base_through_rally::can_nva_place_base_through_rally;
use board::domain::queries::board_level_queries::is_there_any_space_in_which_a_factions_base_is_protected_with_fewer_than_units::is_there_any_space_in_which_a_factions_base_is_protected_with_fewer_than_units;
use board::domain::board::Board;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use game_definitions::flags::Flags;
use game_definitions::forces::Forces;
use players::domain::decision::Decision;
use players::domain::decision_production::produce_decision_to_rally_for_nva::produce_decision_to_rally_for_nva;
use randomization::controllers::randomization_controller_trait::RandomizationControllerTrait;
use randomization::controllers::randomization_controller_trait::RandomizationControllers;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;

fn does_roll_of_two_six_sided_dice_exceed_number_of_nva_guerrillas_available(
    board: &Board,
    randomization_controller: &RandomizationControllers,
) -> Result<bool, String> {
    Ok((randomization_controller.roll_six_sided_die()?
        + randomization_controller.roll_six_sided_die()?)
        > (board.get_forces_available(Forces::UndergroundNvaGuerrilla)?
            + board.get_forces_available(Forces::ActiveNvaGuerrilla)?))
}

pub fn whether_to_rally(
    faction: Factions,
    possible_actions: &[SequenceOfPlaySlots],
    board: &Board,
    flags_controller: &FlagsController,
    randomization_controller: &RandomizationControllers,
) -> Result<Option<Decision>, String> {
    match faction {
        Factions::NVA => {
            // Conditions in which NVA will decide to rally:
            // -Trail is 0 or 1
            // -At least one space with one or more NVA Bases has fewer than 2 NVA units.
            // -A roll of two six-sided dice now exceeds the number of NVA Guerrillas Available.
            // -Rally per the instructions below would place at least one NVA Base.
            // -The NVA attempted to March but we were unable to do so.

            // Sanity check:
            if board.get_faction_stat(FactionStats::NvaResources)? == 0 {
                return Ok(None);
            }

            if board.get_faction_stat(FactionStats::TheTrail)? < MAXIMUM_TRAIL_NUMBER_UNDER_WHICH_NVA_WILL_ATTEMPT_TO_RALLY
                || is_there_any_space_in_which_a_factions_base_is_protected_with_fewer_than_units(
                    Factions::NVA,
                    board,
                    MAXIMUM_NUMBER_OF_NVA_UNITS_ANY_BASE_MUST_BE_PROTECTED_BY_UNDER_WHICH_NVA_WILL_ATTEMPT_TO_RALLY,
                )?
                || does_roll_of_two_six_sided_dice_exceed_number_of_nva_guerrillas_available(
                    board,
                    randomization_controller,
                )?
                || can_nva_place_base_through_rally(board)?
                || flags_controller.has_flag(Flags::NvaAttemptedToMarchButWereUnable)?
            {
                Ok(Some(produce_decision_to_rally_for_nva(
                    possible_actions,
                    board,
                    flags_controller,
                    randomization_controller,
                )?))
            } else {
                Ok(None)
            }
        }
        _ => panic!("Whether to rally not implemented for faction {:?}", faction),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use randomization::controllers::deterministic::randomization_controller_returns_number::RandomizationControllerReturnsNumber;

    use flags::controllers::flags_controller::FlagsController;
    use game_definitions::space_identifiers::SpaceIdentifiers;
    use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;

    #[test]
    fn test_if_nva_doesnt_pass_conditions_to_rally_it_will_not_return_a_decision(
    ) -> Result<(), String> {
        let mut board = Board::new();
        let randomization_controller = RandomizationControllerReturnsNumber::new(1).into();
        let flags_controller = FlagsController::new();

        board.set_faction_stat(FactionStats::TheTrail, 3)?;

        let possible_decision = whether_to_rally(
            Factions::NVA,
            &[SequenceOfPlaySlots::FirstFactionOperationOnly],
            &board,
            &flags_controller,
            &randomization_controller,
        )?;

        assert!(possible_decision.is_none());

        Ok(())
    }

    #[test]
    fn test_if_trail_is_0_or_1_it_produces_a_decision() -> Result<(), String> {
        let mut board = Board::new();
        let randomization_controller = RandomizationControllerReturnsNumber::new(1).into();
        let flags_controller = FlagsController::new();

        board.set_faction_stat(FactionStats::TheTrail, 1)?;
        board.set_faction_stat(FactionStats::NvaResources, 5)?;

        // Need guerrillas and bases in available.
        board.set_forces_in_space(Forces::NvaBase, 6, SpaceIdentifiers::Available)?;
        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            10,
            SpaceIdentifiers::Available,
        )?;

        let possible_decision = whether_to_rally(
            Factions::NVA,
            &[SequenceOfPlaySlots::FirstFactionOperationOnly],
            &board,
            &flags_controller,
            &randomization_controller,
        )?;

        assert!(possible_decision.is_some());

        Ok(())
    }

    #[test]
    fn test_if_a_nva_base_isnt_protected_enough_it_produces_a_decision() -> Result<(), String> {
        let mut board = Board::new();
        let randomization_controller = RandomizationControllerReturnsNumber::new(1).into();
        let flags_controller = FlagsController::new();

        // Need to up the trail or else it always passes.
        board.set_faction_stat(FactionStats::TheTrail, 3)?;
        board.set_faction_stat(FactionStats::NvaResources, 5)?;

        // Need guerrillas and bases in available.
        board.set_forces_in_space(Forces::NvaBase, 6, SpaceIdentifiers::Available)?;
        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            10,
            SpaceIdentifiers::Available,
        )?;

        board.set_forces_in_space(Forces::NvaBase, 2, SpaceIdentifiers::KienHoaVinhBinh)?;
        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            1,
            SpaceIdentifiers::KienHoaVinhBinh,
        )?;

        let possible_decision = whether_to_rally(
            Factions::NVA,
            &[SequenceOfPlaySlots::FirstFactionOperationOnly],
            &board,
            &flags_controller,
            &randomization_controller,
        )?;

        assert!(possible_decision.is_some());

        Ok(())
    }

    #[test]
    fn test_if_a_roll_of_two_six_sided_die_exceeds_the_number_of_nva_guerrillas_available_it_produces_a_decision(
    ) -> Result<(), String> {
        let mut board = Board::new();
        let randomization_controller = RandomizationControllerReturnsNumber::new(4).into();
        let flags_controller = FlagsController::new();

        // Need to up the trail or else it always passes.
        board.set_faction_stat(FactionStats::TheTrail, 3)?;
        board.set_faction_stat(FactionStats::NvaResources, 5)?;

        // Need guerrillas and bases in available.
        board.set_forces_in_space(Forces::NvaBase, 6, SpaceIdentifiers::Available)?;
        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            10,
            SpaceIdentifiers::Available,
        )?;

        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            5,
            SpaceIdentifiers::Available,
        )?;
        board.set_forces_in_space(Forces::ActiveNvaGuerrilla, 2, SpaceIdentifiers::Available)?;

        let possible_decision = whether_to_rally(
            Factions::NVA,
            &[SequenceOfPlaySlots::FirstFactionOperationOnly],
            &board,
            &flags_controller,
            &randomization_controller,
        )?;

        assert!(possible_decision.is_some());

        Ok(())
    }

    #[test]
    fn test_if_rally_per_instructions_would_place_at_least_one_nva_base() -> Result<(), String> {
        let mut board = Board::new();
        let randomization_controller = RandomizationControllerReturnsNumber::new(1).into();
        let flags_controller = FlagsController::new();

        // Need to up the trail or else it always passes.
        board.set_faction_stat(FactionStats::TheTrail, 3)?;
        board.set_faction_stat(FactionStats::NvaResources, 5)?;

        // Need guerrillas and bases in available.
        board.set_forces_in_space(Forces::NvaBase, 6, SpaceIdentifiers::Available)?;
        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            10,
            SpaceIdentifiers::Available,
        )?;

        // For that there must be at least 4 NVA or VC Guerrillas or Troops (including 2 NVA Guerrillas and room for the Base)

        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            1,
            SpaceIdentifiers::BaXuyen,
        )?;
        board.set_forces_in_space(Forces::ActiveNvaGuerrilla, 1, SpaceIdentifiers::BaXuyen)?;
        board.set_forces_in_space(Forces::UndergroundVcGuerrilla, 1, SpaceIdentifiers::BaXuyen)?;
        board.set_forces_in_space(Forces::NvaTroop, 1, SpaceIdentifiers::BaXuyen)?;
        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::BaXuyen)?;

        let possible_decision = whether_to_rally(
            Factions::NVA,
            &[SequenceOfPlaySlots::FirstFactionOperationOnly],
            &board,
            &flags_controller,
            &randomization_controller,
        )?;

        assert!(possible_decision.is_some());

        Ok(())
    }

    #[test]
    fn test_deciding_to_rally_would_contain_sequence_of_play_mutations() -> Result<(), String> {
        let mut board = Board::new();
        let randomization_controller = RandomizationControllerReturnsNumber::new(1).into();
        let flags_controller = FlagsController::new();

        // Need to up the trail or else it always passes.
        board.set_faction_stat(FactionStats::TheTrail, 3)?;
        board.set_faction_stat(FactionStats::NvaResources, 5)?;

        // Need guerrillas and bases in available.
        board.set_forces_in_space(Forces::NvaBase, 6, SpaceIdentifiers::Available)?;
        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            10,
            SpaceIdentifiers::Available,
        )?;

        // For that there must be at least 4 NVA or VC Guerrillas or Troops (including 2 NVA Guerrillas and room for the Base)

        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            1,
            SpaceIdentifiers::BaXuyen,
        )?;
        board.set_forces_in_space(Forces::ActiveNvaGuerrilla, 1, SpaceIdentifiers::BaXuyen)?;
        board.set_forces_in_space(Forces::UndergroundVcGuerrilla, 1, SpaceIdentifiers::BaXuyen)?;
        board.set_forces_in_space(Forces::NvaTroop, 1, SpaceIdentifiers::BaXuyen)?;
        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::BaXuyen)?;

        let possible_decision = whether_to_rally(
            Factions::NVA,
            &[SequenceOfPlaySlots::FirstFactionOperationOnly],
            &board,
            &flags_controller,
            &randomization_controller,
        )?;

        assert!(possible_decision.is_some());
        assert!(possible_decision
            .unwrap()
            .get_mutations()?
            .has_sequence_of_play_mutations()?);

        Ok(())
    }

    #[test]
    fn test_if_nva_attempted_to_march_but_was_unable_to_do_so() -> Result<(), String> {
        let mut board = Board::new();
        let randomization_controller = RandomizationControllerReturnsNumber::new(1).into();

        let mut flags_controller = FlagsController::new();

        flags_controller.set_flag(Flags::NvaAttemptedToMarchButWereUnable)?;

        // Need to up the trail or else it always passes.
        board.set_faction_stat(FactionStats::TheTrail, 3)?;
        board.set_faction_stat(FactionStats::NvaResources, 5)?;

        let possible_decision = whether_to_rally(
            Factions::NVA,
            &[SequenceOfPlaySlots::FirstFactionOperationOnly],
            &board,
            &flags_controller,
            &randomization_controller,
        )?;

        assert!(possible_decision.is_some());

        Ok(())
    }
}
