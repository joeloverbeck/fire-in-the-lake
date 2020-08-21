use board::domain::queries::calculate_number_of_forces_of_a_particular_faction_in_space::calculate_number_of_forces_of_a_particular_faction_in_space;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;

pub fn will_moving_in_number_of_nva_forces_turn_space_into_nva_control(
    number_of_nva_forces: u8,
    occupable_space: &Spaces,
) -> Result<bool, String> {
    // Nva Control gets achieved when NvaTroops outnumber all others (including VC)
    let total_future_number_of_nva_forces =
        calculate_number_of_forces_of_a_particular_faction_in_space(
            Factions::NVA,
            occupable_space,
        )? + number_of_nva_forces;

    Ok(total_future_number_of_nva_forces
        > (calculate_number_of_forces_of_a_particular_faction_in_space(
            Factions::VC,
            occupable_space,
        )? + calculate_number_of_forces_of_a_particular_faction_in_space(
            Factions::ARVN,
            occupable_space,
        )? + calculate_number_of_forces_of_a_particular_faction_in_space(
            Factions::US,
            occupable_space,
        )?))
}
