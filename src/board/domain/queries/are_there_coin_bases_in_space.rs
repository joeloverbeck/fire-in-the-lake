use board::domain::queries::calculate_number_of_coin_bases::calculate_number_of_coin_bases;
use board::domain::space::Spaces;

pub fn are_there_coin_bases_in_space(occupable_space: &Spaces) -> Result<bool, String> {
    Ok(calculate_number_of_coin_bases(occupable_space)? > 0)
}
