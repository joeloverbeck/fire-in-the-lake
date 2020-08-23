use board::domain::board::Board;
use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::forces::Forces;

pub fn is_there_a_minimum_number_of_a_specific_force_in_any_adjacent_space(
    force: Forces,
    minimum_number: u8,
    occupable_space: &Spaces,
    board: &Board,
) -> Result<bool, String> {
    let mut is_there_that_minimum_number_of_a_force_in_an_adjacent_space: bool = false;

    for adjacent_space_identifier in occupable_space.get_adjacent_space_identifiers()?.iter() {
        // get that space.
        let corresponding_space = board.get_space(*adjacent_space_identifier)?;

        if corresponding_space.get_forces(force)? >= minimum_number {
            is_there_that_minimum_number_of_a_force_in_an_adjacent_space = true;
            break;
        }
    }

    Ok(is_there_that_minimum_number_of_a_force_in_an_adjacent_space)
}
