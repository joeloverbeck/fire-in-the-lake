use board::domain::board::Board;
use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use players::domain::decision::Decision;
use players::domain::faction_stats_mutation::FactionStatsMutation;
use players::domain::forces_mutation::ForcesMutation;
use players::domain::mutation_types::MutationTypes;
use players::domain::mutations::Mutations;
use players::domain::mutations_production::produce_as_many_terror_mutations_for_nva_as_possible::produce_as_many_terror_mutations_for_nva_as_possible;
use players::domain::mutations_production::produce_sequence_of_play_mutation_for_operation::produce_sequence_of_play_mutation_for_operation;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use players::domain::space_mutation::SpaceMutation;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use std::collections::VecDeque;

pub fn produce_decision_to_terror_for_nva(
    terror_targets: &mut Vec<&Spaces>,
    board: &Board,
    possible_actions: &[SequenceOfPlaySlots],
) -> Result<Decision, String> {
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

    // Additional instructions: choose spaces to reduce Total Support as much as able.
    // Tie breaker: where no Underground NVA Guerrilla.

    // First will separate the terror targets according to preference.

    let mut best_terror_targets: VecDeque<&Spaces> = VecDeque::new();
    let mut secondary_terror_targets: VecDeque<&Spaces> = VecDeque::new();

    terror_targets.sort_by(|a, b| {
        b.get_total_support()
            .unwrap()
            .cmp(&a.get_total_support().unwrap())
    });

    for terror_target in terror_targets.iter() {
        if terror_target.get_forces(Forces::UndergroundNvaGuerrilla)? == 0 {
            // Goes to best.
            best_terror_targets.push_back(terror_target);
        } else {
            secondary_terror_targets.push_back(terror_target);
        }
    }

    // Now it's just a matter of going through the deques as long as there's any
    // entry in them and the NVA has resources to pull off the terror attacks.
    let mut nva_resources = board.get_faction_stat(FactionStats::NvaResources)?;

    let mut forces_mutations: Vec<ForcesMutation> = Vec::new();
    let mut space_mutations: Vec<SpaceMutation> = Vec::new();

    produce_as_many_terror_mutations_for_nva_as_possible(
        &mut nva_resources,
        &mut best_terror_targets,
        &mut secondary_terror_targets,
        &mut forces_mutations,
        &mut space_mutations,
    )?;

    let mut faction_stats_mutations: Vec<FactionStatsMutation> = Vec::new();

    // If nva_resources at this point is different than the original value, there was a mutation.
    if nva_resources != board.get_faction_stat(FactionStats::NvaResources)? {
        faction_stats_mutations.push(FactionStatsMutation::new(
            FactionStats::NvaResources,
            MutationTypes::Reduce,
            board.get_faction_stat(FactionStats::NvaResources)?,
            nva_resources,
        ));
    }

    // Finally need to produce the sequence of play mutation for playing an operation.
    let mut sequence_of_play_mutations: Vec<SequenceOfPlayMutation> = Vec::new();

    produce_sequence_of_play_mutation_for_operation(
        Factions::NVA,
        possible_actions,
        &mut sequence_of_play_mutations,
    )?;

    let mut mutations = Mutations::new();

    mutations.set_forces_mutations(forces_mutations)?;
    mutations.set_space_mutations(space_mutations)?;
    mutations.set_faction_stats_mutations(faction_stats_mutations)?;
    mutations.set_sequence_of_play_mutations(sequence_of_play_mutations)?;

    Ok(Decision::new(mutations))
}
