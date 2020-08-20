use board::domain::occupable_space::OccupableSpace;
use board::domain::queries::calculate_number_of_vc_guerrillas_in_space::calculate_number_of_vc_guerrillas_in_space;

pub fn are_there_any_vc_guerrillas_in_space(
    occupable_space: &OccupableSpace,
) -> Result<bool, String> {
    Ok(calculate_number_of_vc_guerrillas_in_space(occupable_space)? > 0)
}
