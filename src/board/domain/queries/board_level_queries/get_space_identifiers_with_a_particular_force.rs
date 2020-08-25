use board::domain::board::Board;
use board::domain::queries::space_level_queries::are_there_any_of_a_particular_force_in_space::are_there_any_of_a_particular_force_in_space;
use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn get_space_identifiers_with_a_particular_force<'a>(
    force: Forces,
    board: &'a Board,
) -> Result<Vec<&'a SpaceIdentifiers>, String> {
    Ok(board
        .get_occupable_spaces()?
        .iter()
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
    fn test_can_get_space_identifiers_of_spaces_that_have_a_certain_force() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::AnLoc)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::QuangDucLongKhanh)?;
        board.set_forces_in_space(Forces::UndergroundArvnRanger, 2, SpaceIdentifiers::QuangNam)?;

        let spaces_with_us_bases =
            get_space_identifiers_with_a_particular_force(Forces::UsBase, &board)?;

        assert_eq!(spaces_with_us_bases.len(), 2);
        assert!(spaces_with_us_bases
            .iter()
            .any(|space_identifier| *space_identifier == &SpaceIdentifiers::AnLoc));
        assert!(spaces_with_us_bases
            .iter()
            .any(|space_identifier| *space_identifier == &SpaceIdentifiers::QuangDucLongKhanh));

        Ok(())
    }
}
