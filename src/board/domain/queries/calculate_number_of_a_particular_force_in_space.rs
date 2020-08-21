use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::forces::Forces;

pub fn calculate_number_of_a_particular_force_in_space(
    force: Forces,
    occupable_space: &Spaces,
) -> Result<u8, String> {
    Ok(occupable_space.get_forces(force)?)
}
