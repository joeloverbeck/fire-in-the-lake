extern crate math;
use self::math::round;
use board::domain::space::Space;
use board::domain::space::Spaces;

use game_definitions::forces::Forces;

pub fn calculate_nva_troops_hit_power_for_attack(occupable_space: &Spaces) -> Result<u8, String> {
    let total_number_of_nva_pieces_in_space = occupable_space.get_forces(Forces::NvaTroop)? as f64;

    Ok(round::floor(total_number_of_nva_pieces_in_space / 2., 0) as u8)
}
