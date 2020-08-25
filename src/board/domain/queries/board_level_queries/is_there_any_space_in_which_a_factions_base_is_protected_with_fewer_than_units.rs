use board::domain::board::Board;
use board::domain::queries::space_level_queries::calculate_number_of_a_factions_pieces_minus_bases_in_space::calculate_number_of_a_factions_pieces_minus_bases_in_space;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use board::domain::space::Space;

pub fn is_there_any_space_in_which_a_factions_base_is_protected_with_fewer_than_units(
    faction: Factions,
    board: &Board,
    minimum_number_of_units: u8,
) -> Result<bool, String> {
    Ok(board
        .get_occupable_spaces()?
        .iter()
        .filter(|(_, occupable_space)| match faction {
            Factions::NVA => {
                occupable_space.get_forces(Forces::NvaBase).unwrap()
                    + occupable_space.get_forces(Forces::TunneledNvaBase).unwrap()
                    >= 1
            }
            _ => panic!(
                "Base protection query not handled for faction {:?}",
                faction
            ),
        })
        .any(|(_, occupable_space)| {
            calculate_number_of_a_factions_pieces_minus_bases_in_space(faction, occupable_space)
                .unwrap()
                < minimum_number_of_units
        }))
}
