use board::domain::queries::board_level_queries::get_spaces_with_enough_concentration_of_coin_troops_to_bombard_and_nva_troops_present::get_spaces_with_enough_concentration_of_coin_troops_to_bombard_and_nva_troops_present;
use board::domain::queries::space_level_queries::is_there_a_big_enough_concentration_of_coin_pieces_for_bombardment::is_there_a_big_enough_concentration_of_coin_pieces_for_bombardment;
use board::domain::queries::board_level_queries::is_there_a_minimum_number_of_a_specific_force_in_any_adjacent_space::is_there_a_minimum_number_of_a_specific_force_in_any_adjacent_space;
use board::domain::space::Spaces;
use board::domain::board::Board;
use board::domain::space::Space;
use game_definitions::forces::Forces;

pub fn get_spaces_with_enough_concentration_of_coin_troops_to_bombard_but_nva_troops_only_adjacent<
    'a,
>(
    board: &'a Board,
) -> Result<Vec<&'a Spaces>, String> {
    let bombardment_targets_with_nva_troops_present: Vec<&Spaces> =
        get_spaces_with_enough_concentration_of_coin_troops_to_bombard_and_nva_troops_present(
            board,
        )?;

    Ok(board
        .get_occupable_spaces()?
        .iter()
        .filter(|(_, occupable_space)| {
            !bombardment_targets_with_nva_troops_present
                .iter()
                .any(|other_occupable_spaces| {
                    other_occupable_spaces.get_space_identifier().unwrap()
                        == occupable_space.get_space_identifier().unwrap()
                })
        })
        .filter(|(_, occupable_space)| {
            is_there_a_big_enough_concentration_of_coin_pieces_for_bombardment(occupable_space)
                .unwrap()
        })
        .filter(|(_, occupable_space)| {
            is_there_a_minimum_number_of_a_specific_force_in_any_adjacent_space(
                Forces::NvaTroop,
                3,
                occupable_space,
                board,
            )
            .unwrap()
        })
        .map(|(_, occupable_space)| occupable_space)
        .collect::<Vec<&Spaces>>())
}
