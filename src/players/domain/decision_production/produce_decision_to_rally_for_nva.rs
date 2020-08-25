use board::domain::queries::board_level_queries::would_the_trail_get_improved_through_rally::would_the_trail_get_improved_through_rally;
use sequence_of_play::domain::sequence_of_play_slots::SequenceOfPlaySlots;
use players::domain::mutations_production::produce_sequence_of_play_mutation_for_operation::produce_sequence_of_play_mutation_for_operation;
use players::domain::sequence_of_play_mutation::SequenceOfPlayMutation;
use board::domain::queries::board_level_queries::get_faction_base_spaces_accompanied_with_a_number_or_greater_faction_units::get_faction_base_spaces_accompanied_with_a_number_or_greater_faction_units;
use board::domain::queries::board_level_queries::get_faction_base_spaces_accompanied_with_a_number_or_fewer_faction_units::get_faction_base_spaces_accompanied_with_a_number_or_fewer_faction_units;
use players::domain::mutations_production::produce_unit_placement_rally_mutations_for_nva::produce_unit_placement_rally_mutations_for_nva;
use players::domain::mutations::Mutations;
use players::domain::faction_stats_mutation::FactionStatsMutation;
use board::domain::queries::board_level_queries::get_spaces_with_faction_group_control_in_laos_or_cambodia::get_spaces_with_faction_group_control_in_laos_or_cambodia;
use board::domain::queries::board_level_queries::get_spaces_where_nva_can_place_base_through_rally::get_spaces_where_nva_can_place_base_through_rally;
use std::collections::VecDeque;
use board::domain::space::Spaces;
use board::domain::board::Board;
use flags::controllers::flags_controller::FlagsController;
use game_definitions::event_types::EventTypes;
use game_definitions::faction_groups::FactionGroups;
use game_definitions::nva_capabilities::NvaCapabilities;
use players::domain::decision::Decision;
use randomization::controllers::randomization_controller_trait::RandomizationControllers;
use game_definitions::factions::Factions;
use game_definitions::faction_stats::FactionStats;
use std::iter::FromIterator;
use players::domain::mutation_types::MutationTypes;
use players::domain::decision_information::DecisionInformation;

pub fn produce_decision_to_rally_for_nva(
    possible_actions: &[SequenceOfPlaySlots],
    board: &Board,
    flags_controller: &FlagsController,
    randomization_controller: &RandomizationControllers,
) -> Result<Decision, String> {
    // If the NVA Rally, they do so in up to 3 spaces (without Support, 3.3.1) and improve the Trail. EXCEPTION: if the unshaded
    // "AAA" Capability is in effect and the Trail would be improved per below, they instead Rally in only 1 space and improve the Trail.

    // The trail would be improved: after placing any pieces, pay to improve the Trail, if possible unless the NVA already have fewer than 3 Resources, or unless the Trail is already at 3
    // and NVA have fewer than 9 Resources.
    let number_of_spaces_to_rally_in = {
        if flags_controller.has_nva_capability(NvaCapabilities::AAA)?
            && flags_controller.get_event_type_of_nva_capability(NvaCapabilities::AAA)?
                == &EventTypes::Unshaded
            && would_the_trail_get_improved_through_rally(board)?
        {
            1
        } else {
            3
        }
    };

    // NVA would rally as per the following:
    // 1) Place a NVA Base (replacing 2 NVA Guerrillas, removing Active Guerrillas first), wherever there are at last 4 NVA or VC Guerrillas or Troops (including 2 NVA Guerrillas and room for the base).
    // 2) Then place NVA Guerrillas at any COIN Control in Laos or Cambodia.
    // 3) Next place Guerrillas at NVA Bases where there are no or only 1 NVA units, then at any other NVA Bases (with 2 or more NVA units already).
    // 4) Finally place Guerrillas in other random spaces, until the specified number of Rally spaces is reached (or resources run out).

    let mut spaces_where_nva_can_place_bases: VecDeque<&Spaces> =
        VecDeque::from_iter(get_spaces_where_nva_can_place_base_through_rally(board)?);
    let mut coin_control_spaces: VecDeque<&Spaces> = VecDeque::from_iter(
        get_spaces_with_faction_group_control_in_laos_or_cambodia(FactionGroups::Coin, board)?,
    );
    let mut nva_base_spaces_with_no_or_only_one_nva_units: VecDeque<&Spaces> = VecDeque::from_iter(
        get_faction_base_spaces_accompanied_with_a_number_or_fewer_faction_units(
            Factions::NVA,
            1,
            board,
        )?,
    );
    let mut nva_base_spaces_with_2_or_more_nva_units: VecDeque<&Spaces> = VecDeque::from_iter(
        get_faction_base_spaces_accompanied_with_a_number_or_greater_faction_units(
            Factions::NVA,
            2,
            board,
        )?,
    );

    let (forces_mutations, mut nva_resources) = produce_unit_placement_rally_mutations_for_nva(
        number_of_spaces_to_rally_in,
        &mut spaces_where_nva_can_place_bases,
        &mut coin_control_spaces,
        &mut nva_base_spaces_with_no_or_only_one_nva_units,
        &mut nva_base_spaces_with_2_or_more_nva_units,
        board,
        randomization_controller,
    )?;

    // Improve trail if it can
    // May spend another 2 Resources to improve the trail by 1 box (even if the Rally was a Limited Operation, or selected 0 spaces).
    // Rally to improve the trail costs 2 even if the Rally was free.

    let mut faction_stats_mutations: Vec<FactionStatsMutation> = Vec::new();

    if nva_resources >= 2 {
        nva_resources -= 2;
        faction_stats_mutations.push(FactionStatsMutation::new(
            FactionStats::TheTrail,
            MutationTypes::Increase,
            board.get_faction_stat(FactionStats::TheTrail)?,
            1,
        ));
    }

    // Finally must reduce NVA resources by the amount of resources spent.

    faction_stats_mutations.push(FactionStatsMutation::new(
        FactionStats::NvaResources,
        MutationTypes::Reduce,
        board.get_faction_stat(FactionStats::NvaResources)?,
        board.get_faction_stat(FactionStats::NvaResources)? - nva_resources,
    ));

    // As part of any decision we must also produce the sequence of play mutations.
    let mut sequence_of_play_mutations: Vec<SequenceOfPlayMutation> = Vec::new();

    produce_sequence_of_play_mutation_for_operation(
        Factions::NVA,
        possible_actions,
        &mut sequence_of_play_mutations,
    )?;

    let mut mutations = Mutations::new();

    mutations.set_forces_mutations(forces_mutations)?;
    mutations.set_faction_stats_mutations(faction_stats_mutations)?;
    mutations.set_sequence_of_play_mutations(sequence_of_play_mutations)?;

    Ok(Decision::new(
        mutations,
        Some(DecisionInformation::Rally),
        None,
    ))
}
