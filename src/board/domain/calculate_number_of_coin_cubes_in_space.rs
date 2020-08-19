use board::domain::occupable_space::OccupableSpace;
use game_definitions::forces::Forces;

pub fn calculate_number_of_coin_cubes_in_space(
    occupable_space: &OccupableSpace,
) -> Result<u8, String> {
    Ok(occupable_space.get_forces(Forces::ArvnPolice)?
        + occupable_space.get_forces(Forces::ArvnTroop)?
        + occupable_space.get_forces(Forces::UsTroop)?)
}
