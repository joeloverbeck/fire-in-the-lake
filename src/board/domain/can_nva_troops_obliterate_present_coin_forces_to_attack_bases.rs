use board::domain::calculate_number_of_coin_pieces_minus_bases_in_space::calculate_number_of_coin_pieces_minus_bases_in_space;
use board::domain::calculate_nva_troops_hit_power_for_attack::calculate_nva_troops_hit_power_for_attack;
use board::domain::occupable_space::OccupableSpace;

pub fn can_nva_troops_obliterate_present_coin_forces_to_attack_bases(
    occupable_space: &OccupableSpace,
) -> Result<bool, String> {
    let number_of_enemy_pieces_nva_troops_present_can_remove =
        calculate_nva_troops_hit_power_for_attack(&occupable_space)?;
    Ok(
        calculate_number_of_coin_pieces_minus_bases_in_space(&occupable_space)?
            < number_of_enemy_pieces_nva_troops_present_can_remove,
    )
}
