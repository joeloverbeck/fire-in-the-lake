use board::controllers::queries_controller::QueriesController;
use board::domain::board::Board;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use players::domain::decision_production::produce_decision_to_terror_for_nva::produce_decision_to_terror_for_nva;
use randomization::controllers::randomization_controller_trait::RandomizationControllerTrait;
use randomization::controllers::randomization_controller_trait::RandomizationControllers;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;

pub fn whether_to_terror(
    faction: Factions,
    board: &Board,
    possible_actions: &[SequenceOfPlaySlots],
    randomization_controller: &RandomizationControllers,
) -> Result<Option<Decision>, String> {
    // Terror Operations in Cities or Provinces affect Support and Opposition (1.6) and
    // place Terror markers that hinder future efforts to
    // influence it. On LoCs, they place Sabotage markers that block ARVN
    // Resource earnings (6.2.1). Select any spaces where the executing
    // Faction has at least 1 Underground Guerrilla or, for NVA Terror, NVA
    // Troop cube; pay 1 Resource per Province or City (0 for LoCs).

    // PROCEDURE: Activate 1 of the executing Faction’s Underground
    // Guerrillas in each selected space (if any there).
    // • If the space is a Province or City without a Terror marker, place
    // a Terror marker. If VC, shift 1 level toward Active Opposition
    // (1.6). If NVA, shift any Support 1 level toward Neutral.
    // • If the space is a LoC without a Sabotage marker, place a Sabotage
    // marker.
    // • Do not place a Terror/Sabotage marker if all are already on the
    // map. (There are 15.)

    match faction {
        Factions::NVA => {
            // Note that NVA only terrors provinces or cities. That requires resources.

            if board.get_faction_stat(FactionStats::NvaResources)? == 0 {
                return Ok(None);
            }

            // NVA will only terror if can Terror 1d6+ Population at Support.
            let minimum_population = randomization_controller.roll_six_sided_die()?;

            // Need to gather all spaces that NVA can terror and have that minimum population
            let queries_controller = QueriesController::new();

            let mut terror_targets = queries_controller
                .get_spaces_nva_can_terror_and_have_minimum_population(minimum_population, board)?;

            Ok(Some(produce_decision_to_terror_for_nva(
                &mut terror_targets,
                board,
                possible_actions,
            )?))
        }
        _ => panic!(
            "Whether to terror not implemented for faction {:?}",
            faction
        ),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.

    use super::*;
    use randomization::controllers::deterministic::randomization_controller_returns_number::RandomizationControllerReturnsNumber;

    use game_definitions::forces::Forces;
    use game_definitions::space_identifiers::SpaceIdentifiers;
    use game_definitions::support_levels::SupportLevels;
    use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;

    #[test]
    fn test_will_not_decide_to_terror_if_it_doesnt_have_any_nva_guerrillas_or_troops(
    ) -> Result<(), String> {
        let possible_actions: Vec<SequenceOfPlaySlots> =
            vec![SequenceOfPlaySlots::SecondFactionLimitedOperation];

        let possible_decision = whether_to_terror(
            Factions::NVA,
            &Board::new(),
            &possible_actions,
            &RandomizationControllerReturnsNumber::new(1).into(),
        )?;

        assert!(possible_decision.is_none());

        Ok(())
    }

    #[test]
    fn test_will_terror_if_has_nva_troop_at_province_with_support() -> Result<(), String> {
        let mut board = Board::new();

        board.set_faction_stat(FactionStats::NvaResources, 1)?;
        board.set_forces_in_space(Forces::NvaTroop, 1, SpaceIdentifiers::KienPhong)?;
        board.set_support_level_of_space(
            SupportLevels::PassiveSupport,
            SpaceIdentifiers::KienPhong,
        )?;

        let possible_actions: Vec<SequenceOfPlaySlots> =
            vec![SequenceOfPlaySlots::SecondFactionLimitedOperation];

        let possible_decision = whether_to_terror(
            Factions::NVA,
            &board,
            &possible_actions,
            &RandomizationControllerReturnsNumber::new(1).into(),
        )?;

        assert!(possible_decision.is_some());

        let decision = possible_decision.unwrap();

        assert!(decision.get_mutations()?.has_sequence_of_play_mutations()?);
        assert!(decision.get_mutations()?.has_forces_mutations()?);
        assert!(decision.get_mutations()?.has_faction_stats_mutations()?);
        assert!(decision.get_mutations()?.has_space_mutations()?);

        Ok(())
    }
}
