use board::domain::space::Spaces;
use board::domain::board::Board;
use board::domain::queries::space_level_queries::is_there_a_big_enough_concentration_of_coin_pieces_for_bombardment::is_there_a_big_enough_concentration_of_coin_pieces_for_bombardment;
use game_definitions::forces::Forces;
use board::domain::space::Space;

pub fn get_spaces_with_enough_concentration_of_coin_troops_to_bombard_and_nva_troops_present<'a>(
    board: &'a Board,
) -> Result<Vec<&'a Spaces>, String> {
    // Rules: Bombard 2 spaces (in or adjacent to 1 or more spaces with at least 3 NVA Troops, and with either at least 3 COIN
    // troops or a COIN Base and at least 1 COIN Troop).

    Ok(board
        .get_occupable_spaces()?
        .iter()
        .filter(|(_, occupable_space)| {
            is_there_a_big_enough_concentration_of_coin_pieces_for_bombardment(occupable_space)
                .unwrap()
        })
        .filter(|(_, occupable_space)| occupable_space.get_forces(Forces::NvaTroop).unwrap() >= 3)
        .map(|(_, occupable_space)| occupable_space)
        .collect::<Vec<&Spaces>>())
}
