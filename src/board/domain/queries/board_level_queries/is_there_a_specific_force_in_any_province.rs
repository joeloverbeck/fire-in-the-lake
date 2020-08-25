use board::domain::queries::board_level_queries::get_space_identifiers_with_a_particular_force_and_terrain_type::get_space_identifiers_with_a_particular_force_and_terrain_type;
use game_definitions::forces::Forces;
use board::domain::board::Board;
use game_definitions::terrain_types::TerrainTypes;

pub fn is_there_a_specific_force_in_any_province(
    force: Forces,
    board: &Board,
) -> Result<bool, String> {
    // Provinces are either Highlands, Lowlands or Jungles.

    Ok(
        !get_space_identifiers_with_a_particular_force_and_terrain_type(
            force,
            TerrainTypes::Lowland,
            board,
        )?
        .is_empty()
            || !get_space_identifiers_with_a_particular_force_and_terrain_type(
                force,
                TerrainTypes::Highland,
                board,
            )?
            .is_empty()
            || !get_space_identifiers_with_a_particular_force_and_terrain_type(
                force,
                TerrainTypes::Jungle,
                board,
            )?
            .is_empty(),
    )
}

#[cfg(test)]

mod tests {

    use super::*;
    use game_definitions::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_wont_say_there_is_a_force_in_a_province_if_it_is_in_a_city() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::Saigon)?;

        assert!(!is_there_a_specific_force_in_any_province(
            Forces::NvaTroop,
            &board
        )?);

        Ok(())
    }

    #[test]
    fn test_can_find_if_there_is_a_specific_force_in_a_province() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::TheFishhook)?;

        assert!(is_there_a_specific_force_in_any_province(
            Forces::NvaTroop,
            &board
        )?);

        Ok(())
    }
}
