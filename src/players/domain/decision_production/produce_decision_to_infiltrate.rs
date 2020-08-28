use board::domain::board::Board;
use board::domain::queries::space_level_queries::get_spaces_where_nva_can_infiltrate_a_vc_base::get_spaces_where_nva_can_infiltrate_a_vc_base;
use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;
use players::domain::decision::Decision;
use players::domain::decision_information::DecisionInformation;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::mutation_types::MutationTypes;
use players::domain::mutations::Mutations;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use sequence_of_play::domain::slot_occupancy::SlotOccupancy;
use std::collections::VecDeque;
use std::iter::FromIterator;

pub fn produce_decision_to_infiltrate(
    possible_previous_decision: Option<&Decision>,
    _possible_actions: &[SequenceOfPlaySlots],
    times_it_can_infiltrate: u8,
    board: &Board,
) -> Result<Decision, String> {
    // -First replace VC bases in spaces with more NVA than VC pieces but no NVA Base already (Shifting Opposition, 4.4.1),
    // first tunneled bases, within that priority first in spaces with the highest population, finally in random spaces.
    // -If there are no NVA Bases available, they may first remove one base from the space in North Vietnam, Laos or cambodia
    // -With the most NVA pieces to Available in order to replace one VC Base.

    // -Then place Troops (at NVA Bases), first in those Base spaces with the most NVA Guerrillas, within that priority first
    // in spaces with the most NVA Guerrillas, within that priority first in spaces with 2 NVA Bases, and within those priorities
    // first in or adjacent to Cities or Provinces with no NVA Control.
    // If placing Troops, then also replace NVA Guerrillas in those spaces with Troops, but only where there are more than 2
    // NVA Guerrillas in a space and until 2 are left.

    let mut spaces_where_nva_can_infiltrate_a_vc_base =
        get_spaces_where_nva_can_infiltrate_a_vc_base(board)?;

    spaces_where_nva_can_infiltrate_a_vc_base.sort_by(|a, b| {
        b.get_population()
            .unwrap()
            .cmp(&a.get_population().unwrap())
    });

    // Need to order this through the priorities.

    let mut spaces_with_tunneled_bases_sorted_by_highest_population: VecDeque<&Spaces> =
        VecDeque::from_iter(
            spaces_where_nva_can_infiltrate_a_vc_base
                .iter()
                .filter(|space| space.get_forces(Forces::TunneledVcBase).unwrap() > 0)
                .copied(),
        );

    let mut spaces_without_tunneled_bases_sorted_by_highest_population: VecDeque<&Spaces> =
        VecDeque::from_iter(
            spaces_where_nva_can_infiltrate_a_vc_base
                .iter()
                .filter(|space| space.get_forces(Forces::TunneledVcBase).unwrap() == 0)
                .copied(),
        );

    let mut mut_times_it_can_infiltrate = times_it_can_infiltrate;

    let mut forces_mutations: Vec<ForcesMutation> = Vec::new();

    while mut_times_it_can_infiltrate > 0 {
        if !spaces_with_tunneled_bases_sorted_by_highest_population.is_empty()
            || !spaces_without_tunneled_bases_sorted_by_highest_population.is_empty()
        {
            // We are going to infiltrate a vc base.
            if board.get_forces_available(Forces::NvaBase)? == 0 {
                panic!("Case not handled: NVA should replace a VC Base, and yet there are no Nva Bases available. It should first remove one base from another space. Check comments.");
            }
        }

        if !spaces_with_tunneled_bases_sorted_by_highest_population.is_empty() {
            let space = spaces_with_tunneled_bases_sorted_by_highest_population
                .pop_front()
                .unwrap();

            // Gotta make the mutations for it.
            forces_mutations.push(ForcesMutation::new(
                Forces::TunneledVcBase,
                MutationTypes::Reduce,
                1,
                Some(space.get_space_identifier()?),
                Some(SpaceIdentifiers::Available),
            ));
            forces_mutations.push(ForcesMutation::new(
                Forces::TunneledNvaBase,
                MutationTypes::Increase,
                1,
                Some(space.get_space_identifier()?),
                Some(SpaceIdentifiers::Available),
            ));
        } else if !spaces_without_tunneled_bases_sorted_by_highest_population.is_empty() {
            let space = spaces_without_tunneled_bases_sorted_by_highest_population
                .pop_front()
                .unwrap();

            // Gotta make the mutations for it.
            forces_mutations.push(ForcesMutation::new(
                Forces::VcBase,
                MutationTypes::Reduce,
                1,
                Some(space.get_space_identifier()?),
                Some(SpaceIdentifiers::Available),
            ));
            forces_mutations.push(ForcesMutation::new(
                Forces::NvaBase,
                MutationTypes::Increase,
                1,
                Some(space.get_space_identifier()?),
                Some(SpaceIdentifiers::Available),
            ));
        }

        mut_times_it_can_infiltrate -= 1;
    }

    let mut mutations = Mutations::new();

    mutations.set_forces_mutations(forces_mutations)?;

    // Finally should set the complete sequence of play mutations.
    // TODO: Should do a generic version of this.
    let sequence_of_play_mutations: Vec<SequenceOfPlayMutation> =
        vec![SequenceOfPlayMutation::new(
            SequenceOfPlaySlots::FirstFactionOperationPlusSpecialActivity,
            SlotOccupancy::Occupied,
            Factions::NVA,
        )];

    mutations.set_sequence_of_play_mutations(sequence_of_play_mutations)?;

    // TODO: I think this "merging" is already, or partly, implemented
    // in "produce_decision_for_special_activity".

    // Might be that there was a previous decision, probably consisting of
    // data about the main operation.
    let previous_decision_operation_information = {
        if possible_previous_decision.is_some() {
            Some(*possible_previous_decision.unwrap().get_main_action()?)
        } else {
            None
        }
    };

    let decision = Decision::new(
        mutations,
        previous_decision_operation_information,
        Some(DecisionInformation::Infiltrate),
    );

    Ok(decision)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_produces_expected_base_placement_mutations_through_infiltrate() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::TunneledVcBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundVcGuerrilla, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::NvaTroop, 2, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundNvaGuerrilla, 1, SpaceIdentifiers::Saigon)?;

        board.set_forces_in_space(Forces::VcBase, 1, SpaceIdentifiers::QuangDucLongKhanh)?;
        board.set_forces_in_space(
            Forces::UndergroundVcGuerrilla,
            1,
            SpaceIdentifiers::QuangDucLongKhanh,
        )?;
        board.set_forces_in_space(Forces::NvaTroop, 1, SpaceIdentifiers::QuangDucLongKhanh)?;
        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            2,
            SpaceIdentifiers::QuangDucLongKhanh,
        )?;

        board.set_forces_in_space(Forces::NvaBase, 2, SpaceIdentifiers::Available)?;

        let possible_actions = [SequenceOfPlaySlots::FirstFactionOperationPlusSpecialActivity];

        let operation_decision =
            Decision::new(Mutations::new(), Some(DecisionInformation::Rally), None);

        // In order to replace vc bases, there must be a vc base and more nva than vc pieces, but no nva base.
        let sut = produce_decision_to_infiltrate(
            Some(&operation_decision),
            &possible_actions,
            2,
            &board,
        )?;

        assert!(sut.get_mutations()?.has_forces_mutations()?);

        let forces_mutations = sut.get_mutations()?.get_forces_mutations()?;

        assert!(forces_mutations
            .iter()
            .any(|forces_mutation| forces_mutation.get_forces() == &Forces::TunneledNvaBase));
        assert!(forces_mutations
            .iter()
            .any(|forces_mutation| forces_mutation.get_forces() == &Forces::NvaBase));
        assert!(forces_mutations
            .iter()
            .any(|forces_mutation| forces_mutation.get_forces() == &Forces::TunneledVcBase));
        assert!(forces_mutations
            .iter()
            .any(|forces_mutation| forces_mutation.get_forces() == &Forces::VcBase));

        // There should also be sequence of play mutations.

        assert!(sut.get_mutations()?.has_sequence_of_play_mutations()?);

        let sequence_of_play_mutation = sut.get_mutations()?.get_sequence_of_play_mutations()?[0];

        assert_eq!(
            sequence_of_play_mutation.get_sequence_of_play_slot(),
            &SequenceOfPlaySlots::FirstFactionOperationPlusSpecialActivity
        );

        assert_eq!(sut.get_main_action()?, &DecisionInformation::Rally);

        // the decision should also have marked that Infiltrate was chosen.
        assert_eq!(
            sut.get_secondary_action()?,
            &DecisionInformation::Infiltrate
        );

        Ok(())
    }
}
