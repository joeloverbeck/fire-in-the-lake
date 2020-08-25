use randomization::controllers::randomization_controller_trait::RandomizationControllers;
use board::domain::space::Spaces;
use std::collections::VecDeque;
use board::domain::board::Board;
use randomization::domain::random_space_identifier_chooser::RandomSpaceIdentifierChooser;
use players::domain::mutations_production::produce_mutations_for_placing_nva_guerrillas_through_rally::produce_mutations_for_placing_nva_guerrillas_through_rally;
use players::domain::mutations_production::produce_mutations_for_placing_nva_base_through_rally::produce_mutations_for_placing_nva_base_through_rally;
use players::domain::forces_mutation::ForcesMutation;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::forces::Forces;
use game_definitions::faction_stats::FactionStats;

pub fn produce_unit_placement_rally_mutations_for_nva(
    number_of_spaces_to_rally_in: u8,
    spaces_where_nva_can_place_bases: &mut VecDeque<&Spaces>,
    coin_control_spaces: &mut VecDeque<&Spaces>,
    nva_base_spaces_with_no_or_only_one_nva_units: &mut VecDeque<&Spaces>,
    nva_base_spaces_with_2_or_more_nva_units: &mut VecDeque<&Spaces>,
    board: &Board,
    randomization_controller: &RandomizationControllers,
) -> Result<(Vec<ForcesMutation>, u8), String> {
    let mut nva_resources = board.get_faction_stat(FactionStats::NvaResources)?;
    let mut available_guerrillas = board
        .get_forces_in_space(Forces::UndergroundNvaGuerrilla, SpaceIdentifiers::Available)?
        + board.get_forces_in_space(Forces::ActiveNvaGuerrilla, SpaceIdentifiers::Available)?;
    let mut available_nva_bases =
        board.get_forces_in_space(Forces::NvaBase, SpaceIdentifiers::Available)?;

    let mut forces_mutations: Vec<ForcesMutation> = Vec::new();

    let mut mutable_number_of_spaces_to_rally_in = number_of_spaces_to_rally_in;

    // 1 Nva Resource per space.
    while mutable_number_of_spaces_to_rally_in > 0 && nva_resources > 0 {
        // Go through each vec deque in order and attempt to do what each vecdeque is made to do (first one create a base, second one
        // place NVA Guerrillas at COIN control in Laos or Cambodia, third and fourth place guerrillas at NVA bases, fifth random locations).

        if !spaces_where_nva_can_place_bases.is_empty() && available_nva_bases > 0 {
            produce_mutations_for_placing_nva_base_through_rally(
                &mut forces_mutations,
                spaces_where_nva_can_place_bases.pop_front().unwrap(),
            )?;
            nva_resources -= 1;
            available_nva_bases -= 1;
        } else if !coin_control_spaces.is_empty() && available_guerrillas > 0 {
            let spent_nva_guerrillas = produce_mutations_for_placing_nva_guerrillas_through_rally(
                &mut forces_mutations,
                available_guerrillas,
                coin_control_spaces.pop_front().unwrap(),
                board,
            )?;
            nva_resources -= 1;
            available_guerrillas -= spent_nva_guerrillas;
        } else if !nva_base_spaces_with_no_or_only_one_nva_units.is_empty()
            && available_guerrillas > 0
        {
            let spent_nva_guerrillas = produce_mutations_for_placing_nva_guerrillas_through_rally(
                &mut forces_mutations,
                available_guerrillas,
                nva_base_spaces_with_no_or_only_one_nva_units
                    .pop_front()
                    .unwrap(),
                board,
            )?;
            nva_resources -= 1;
            available_guerrillas -= spent_nva_guerrillas;
        } else if !nva_base_spaces_with_2_or_more_nva_units.is_empty() && available_guerrillas > 0 {
            let spent_nva_guerrillas = produce_mutations_for_placing_nva_guerrillas_through_rally(
                &mut forces_mutations,
                available_guerrillas,
                nva_base_spaces_with_2_or_more_nva_units
                    .pop_front()
                    .unwrap(),
                board,
            )?;
            nva_resources -= 1;
            available_guerrillas -= spent_nva_guerrillas;
        } else if nva_resources > 0 && available_guerrillas > 0 {
            let random_space_identifier_chooser = RandomSpaceIdentifierChooser::new();

            let (chosen_space_identifier, _) = random_space_identifier_chooser
                .choose_random_space_identifier(randomization_controller)?;

            let spent_nva_guerrillas = produce_mutations_for_placing_nva_guerrillas_through_rally(
                &mut forces_mutations,
                available_guerrillas,
                board.get_space(*chosen_space_identifier)?,
                board,
            )?;
            nva_resources -= 1;
            available_guerrillas -= spent_nva_guerrillas;
        }

        mutable_number_of_spaces_to_rally_in -= 1;
    }

    Ok((forces_mutations, nva_resources))
}
