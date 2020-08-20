use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::forces::Forces;

pub fn calculate_number_of_vc_guerrillas_in_space(occupable_space: &Spaces) -> Result<u8, String> {
    Ok(occupable_space.get_forces(Forces::UndergroundVcGuerrilla)?
        + occupable_space.get_forces(Forces::ActiveVcGuerrilla)?)
}