use board::domain::queries::calculate_number_of_a_particular_force_in_space::calculate_number_of_a_particular_force_in_space;
use board::domain::space::Spaces;
use game_definitions::forces::Forces;

pub fn are_there_any_of_a_particular_force_in_space(
    force: Forces,
    occupable_space: &Spaces,
) -> Result<bool, String> {
    Ok(calculate_number_of_a_particular_force_in_space(force, occupable_space)? > 0)
}
