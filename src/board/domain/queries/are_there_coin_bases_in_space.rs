use board::domain::occupable_space::OccupableSpace;
use board::domain::queries::calculate_number_of_coin_bases::calculate_number_of_coin_bases;

pub fn are_there_coin_bases_in_space(occupable_space: &OccupableSpace) -> Result<bool, String> {
    Ok(calculate_number_of_coin_bases(occupable_space)? > 0)
}
