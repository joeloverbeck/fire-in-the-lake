use board::domain::occupable_space::OccupableSpace;
use game_definitions::forces::Forces;

pub fn calculate_number_of_coin_bases(occupable_space: &OccupableSpace) -> Result<u8, String> {
    Ok(occupable_space.get_forces(Forces::ArvnBase)?
        + occupable_space.get_forces(Forces::UsBase)?)
}
