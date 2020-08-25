use board::domain::board::Board;
use board::domain::queries::space_level_queries::are_there_any_of_a_particular_force_in_space::are_there_any_of_a_particular_force_in_space;
use board::domain::space::Space;
use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::terrain_types::TerrainTypes;

pub fn get_space_identifiers_with_a_particular_force_and_terrain_type<'a>(
    force: Forces,
    terrain_type: TerrainTypes,
    board: &'a Board,
) -> Result<Vec<&'a SpaceIdentifiers>, String> {
    Ok(board
        .get_occupable_spaces()?
        .iter()
        .filter(|(_, occupable_space)| occupable_space.is_habitable().unwrap())
        .filter(|(_, occupable_space)| occupable_space.get_terrain_type().unwrap() == &terrain_type)
        .filter(|(_, occupable_space)| {
            are_there_any_of_a_particular_force_in_space(force, &occupable_space).unwrap()
        })
        .map(|(space_identifier, _)| space_identifier)
        .collect::<Vec<&SpaceIdentifiers>>())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_when_asking_to_get_space_identifiers_with_a_specific_force_and_terrain_type_it_will_return_expected_space_identifier_if_there_are_coincidences(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::TheParrotsBeak)?;

        let space_identifiers = get_space_identifiers_with_a_particular_force_and_terrain_type(
            Forces::UsTroop,
            TerrainTypes::Jungle,
            &board,
        )?;

        assert_eq!(space_identifiers.len(), 1);
        assert_eq!(space_identifiers[0], &SpaceIdentifiers::TheParrotsBeak);

        Ok(())
    }

    #[test]
    fn test_when_asking_to_get_space_identifiers_with_a_specific_force_and_terrain_type_it_will_return_an_empty_vec_if_there_are_no_coincidences(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::TheParrotsBeak)?;

        let space_identifiers = get_space_identifiers_with_a_particular_force_and_terrain_type(
            Forces::UsTroop,
            TerrainTypes::Highland,
            &board,
        )?;

        assert_eq!(space_identifiers.len(), 0);

        Ok(())
    }
}
