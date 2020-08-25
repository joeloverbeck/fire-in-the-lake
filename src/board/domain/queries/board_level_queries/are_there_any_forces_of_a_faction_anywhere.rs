use game_definitions::factions::Factions;
use board::domain::board::Board;
use board::domain::queries::space_level_queries::calculate_number_of_forces_of_a_particular_faction_in_space::calculate_number_of_forces_of_a_particular_faction_in_space;

pub fn are_there_any_forces_of_a_faction_anywhere(
    faction: Factions,
    board: &Board,
) -> Result<bool, String> {
    let sum: u8 = board
        .get_occupable_spaces()?
        .iter()
        .map(|(_, occupable_space)| {
            calculate_number_of_forces_of_a_particular_faction_in_space(faction, occupable_space)
                .unwrap()
        })
        .sum();

    Ok(sum > 0)
}

#[cfg(test)]
mod tests {

    use super::*;
    use game_definitions::forces::Forces;
    use game_definitions::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_can_find_out_if_there_are_any_pieces_of_a_faction_anywhere() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::ActiveUsIrregular, 1, SpaceIdentifiers::BinhDinh)?;

        assert!(are_there_any_forces_of_a_faction_anywhere(
            Factions::US,
            &board
        )?);

        Ok(())
    }

    #[test]
    fn test_cant_find_any_pieces_of_a_faction_anywhere_if_there_arent_any() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::ArvnPolice, 1, SpaceIdentifiers::BinhDinh)?;

        assert_eq!(
            are_there_any_forces_of_a_faction_anywhere(Factions::US, &board)?,
            false
        );

        Ok(())
    }
}
