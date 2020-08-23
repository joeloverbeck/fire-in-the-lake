use board::domain::queries::calculate_number_of_coin_bases::calculate_number_of_coin_bases;
use board::domain::queries::calculate_number_of_coin_cubes_in_space::calculate_number_of_coin_cubes_in_space;
use board::domain::space::Spaces;

pub fn is_there_a_big_enough_concentration_of_coin_pieces_for_bombardment(
    occupable_space: &Spaces,
) -> Result<bool, String> {
    Ok(
        calculate_number_of_coin_cubes_in_space(occupable_space).unwrap() >= 3
            || (calculate_number_of_coin_bases(occupable_space).unwrap() >= 1
                && calculate_number_of_coin_cubes_in_space(occupable_space).unwrap() >= 1),
    )
}
