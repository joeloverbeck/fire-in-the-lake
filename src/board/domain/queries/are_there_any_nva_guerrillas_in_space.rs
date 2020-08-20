use board::domain::queries::calculate_number_of_nva_guerrillas_in_space::calculate_number_of_nva_guerrillas_in_space;
use board::domain::space::Spaces;

pub fn are_there_any_nva_guerrillas_in_space(occupable_space: &Spaces) -> Result<bool, String> {
    Ok(calculate_number_of_nva_guerrillas_in_space(occupable_space)? > 0)
}
