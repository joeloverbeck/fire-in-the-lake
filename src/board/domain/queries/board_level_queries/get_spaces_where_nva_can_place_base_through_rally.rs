use board::domain::queries::space_level_queries::calculate_number_of_a_factions_pieces_minus_bases_in_space::calculate_number_of_a_factions_pieces_minus_bases_in_space;
use board::domain::space::Spaces;
use board::domain::space::Space;
use board::domain::queries::space_level_queries::calculate_number_of_a_particular_force_in_space::calculate_number_of_a_particular_force_in_space;
use board::domain::board::Board;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;

pub fn get_spaces_where_nva_can_place_base_through_rally<'a>(
    board: &'a Board,
) -> Result<Vec<&'a Spaces>, String> {
    Ok(board
        .get_occupable_spaces()?
        .iter()
        .filter(|(_, occupable_space)| {
            calculate_number_of_a_particular_force_in_space(
                Forces::ActiveNvaGuerrilla,
                occupable_space,
            )
            .unwrap()
                + calculate_number_of_a_particular_force_in_space(
                    Forces::UndergroundNvaGuerrilla,
                    occupable_space,
                )
                .unwrap()
                >= 2
        })
        .filter(|(_, occupable_space)| {
            occupable_space.get_forces(Forces::NvaBase).unwrap()
                + occupable_space.get_forces(Forces::TunneledNvaBase).unwrap()
                < 2
        })
        .filter(|(_, occupable_space)| {
            calculate_number_of_a_factions_pieces_minus_bases_in_space(
                Factions::NVA,
                occupable_space,
            )
            .unwrap()
                + calculate_number_of_a_factions_pieces_minus_bases_in_space(
                    Factions::VC,
                    occupable_space,
                )
                .unwrap()
                >= 4
        })
        .map(|(_, occupable_space)| occupable_space)
        .collect::<Vec<&Spaces>>())
}
