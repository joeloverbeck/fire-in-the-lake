use board::domain::queries::space_level_queries::calculate_number_of_faction_group_bases_in_space::calculate_number_of_faction_group_bases_in_space;
use board::domain::space::Spaces;
use game_definitions::faction_groups::FactionGroups;

pub fn are_there_faction_group_bases_in_space(
    faction_group: FactionGroups,
    occupable_space: &Spaces,
) -> Result<bool, String> {
    Ok(calculate_number_of_faction_group_bases_in_space(faction_group, occupable_space)? > 0)
}
