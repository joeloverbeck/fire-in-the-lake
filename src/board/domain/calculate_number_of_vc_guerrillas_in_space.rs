use board::domain::occupable_space::OccupableSpace;
use game_definitions::forces::Forces;

pub fn calculate_number_of_vc_guerrillas_in_space(
    occupable_space: &OccupableSpace,
) -> Result<u8, String> {
    Ok(occupable_space.get_forces(Forces::UndergroundVcGuerrilla)?
        + occupable_space.get_forces(Forces::ActiveVcGuerrilla)?)
}
