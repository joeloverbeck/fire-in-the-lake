use board::domain::board::Board;
use board::domain::queries::space_level_queries::are_there_faction_bases_in_space::are_there_faction_bases_in_space;
use board::domain::queries::space_level_queries::calculate_number_of_forces_of_a_particular_faction_in_space::calculate_number_of_forces_of_a_particular_faction_in_space;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;

pub fn get_spaces_where_nva_can_infiltrate_a_vc_base(
    board: &Board,
) -> Result<Vec<&Spaces>, String> {
    Ok(board
        .get_occupable_spaces()?
        .iter()
        .filter(|(_, occupable_space)| {
            !are_there_faction_bases_in_space(Factions::NVA, occupable_space).unwrap()
        })
        .filter(|(_, occupable_space)| {
            are_there_faction_bases_in_space(Factions::VC, occupable_space).unwrap()
        })
        .filter(|(_, occupable_space)| {
            calculate_number_of_forces_of_a_particular_faction_in_space(
                Factions::NVA,
                occupable_space,
            )
            .unwrap()
                > calculate_number_of_forces_of_a_particular_faction_in_space(
                    Factions::VC,
                    occupable_space,
                )
                .unwrap()
        })
        .map(|(_, occupable_space)| occupable_space)
        .collect::<Vec<&Spaces>>())
}
