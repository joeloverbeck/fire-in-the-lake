use board::domain::board::Board;
use board::domain::space::Space;
use game_definitions::forces::Forces;
use game_definitions::terrain_types::TerrainTypes;

pub fn is_there_a_specific_force_in_a_terrain_type(
    force: Forces,
    terrain_type: TerrainTypes,
    board: &Board,
) -> Result<bool, String> {
    Ok(board
        .get_occupable_spaces()?
        .iter()
        .any(|(_, occupable_space)| {
            occupable_space.is_habitable().unwrap()
                && occupable_space.get_terrain_type().unwrap() == &terrain_type
                && occupable_space.get_forces(force).unwrap() > 0
        }))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use board::domain::queries::board_level_queries::is_there_a_faction_group_base_in_a_province_with_less_or_equal_cubes_of_that_group::is_there_a_faction_group_base_in_a_province_with_less_or_equal_cubes_of_that_group;
use super::*;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::faction_groups::FactionGroups;

    #[test]
    fn test_if_asks_about_a_force_being_in_a_terrain_type_it_says_there_is_if_that_force_is_in_such_a_terrain_type(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::TheFishhook)?;

        assert!(is_there_a_specific_force_in_a_terrain_type(
            Forces::NvaTroop,
            TerrainTypes::Jungle,
            &board
        )?);

        Ok(())
    }

    #[test]
    fn test_if_asks_about_a_force_being_in_a_terrain_type_it_says_no_if_that_force_is_not_in_that_terrain_type(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::TheFishhook)?;

        assert!(!is_there_a_specific_force_in_a_terrain_type(
            Forces::NvaTroop,
            TerrainTypes::Highland,
            &board
        )?);

        Ok(())
    }

    #[test]
    fn test_can_query_whether_there_is_a_base_of_a_faction_ground_in_a_province_with_less_or_equal_number_of_that_groups_cubes(
    ) -> Result<(), String> {
        // Regarding the original event: It's only effective in its unshaded part if there's a COIN base in a province with less or equal
        // COIN "cubes" than a number specifically (US to Casualties).
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::ArvnPolice, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::Saigon)?;

        assert_eq!(
            is_there_a_faction_group_base_in_a_province_with_less_or_equal_cubes_of_that_group(
                &FactionGroups::Coin,
                2,
                &board
            )?,
            true
        );

        Ok(())
    }
}
