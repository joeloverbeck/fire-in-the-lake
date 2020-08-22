use board::domain::queries::space_level_queries::calculate_number_of_faction_cubes_in_space::calculate_number_of_faction_cubes_in_space;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;

pub fn calculate_number_of_coin_cubes_in_space(occupable_space: &Spaces) -> Result<u8, String> {
    Ok(
        calculate_number_of_faction_cubes_in_space(Factions::ARVN, occupable_space)?
            + calculate_number_of_faction_cubes_in_space(Factions::US, occupable_space)?,
    )
}
