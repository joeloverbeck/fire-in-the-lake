use board::domain::board::Board;
use board::domain::space::Space;
use game_definitions::forces::Forces;

pub fn is_there_a_specific_force_anywhere(force: Forces, board: &Board) -> Result<bool, String> {
    Ok(board
        .get_occupable_spaces()?
        .iter()
        .any(|(_, occupable_space)| occupable_space.get_forces(force).unwrap() > 0))
}

#[cfg(test)]

mod tests {

    use super::*;
    use game_definitions::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_can_find_out_if_theres_a_specific_force_anywhere() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ActiveNvaGuerrilla, 1, SpaceIdentifiers::Kontum)?;

        assert!(is_there_a_specific_force_anywhere(
            Forces::ActiveNvaGuerrilla,
            &board
        )?);

        Ok(())
    }
}
