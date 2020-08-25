use board::domain::queries::space_level_queries::calculate_number_of_a_factions_pieces_minus_bases_in_space::calculate_number_of_a_factions_pieces_minus_bases_in_space;
use board::domain::space::Spaces;
use board::domain::space::Space;
use board::domain::board::Board;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;

pub fn get_faction_base_spaces_accompanied_with_a_number_or_fewer_faction_units(
    faction: Factions,
    max_number_units: u8,
    board: &Board,
) -> Result<Vec<&Spaces>, String> {
    Ok(board
        .get_occupable_spaces()?
        .iter()
        .filter(|(_, occupable_space)| match faction {
            Factions::NVA => {
                occupable_space.get_forces(Forces::NvaBase).unwrap()
                    + occupable_space.get_forces(Forces::TunneledNvaBase).unwrap()
                    >= 1
            }
            _ => panic!("Not implemented for faction {:?}", faction),
        })
        .filter(|(_, occupable_space)| {
            calculate_number_of_a_factions_pieces_minus_bases_in_space(faction, occupable_space)
                .unwrap()
                <= max_number_units
        })
        .map(|(_, occupable_space)| occupable_space)
        .collect::<Vec<&Spaces>>())
}
