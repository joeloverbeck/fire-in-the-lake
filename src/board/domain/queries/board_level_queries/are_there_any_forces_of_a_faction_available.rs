use board::domain::board::Board;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;

pub fn are_there_any_forces_of_a_faction_available(
    faction: Factions,
    board: &Board,
) -> Result<bool, String> {
    match faction {
        Factions::US => Ok(board.get_forces_available(Forces::UsTroop)? > 0
            || board.get_forces_available(Forces::ActiveUsIrregular)? > 0
            || board.get_forces_available(Forces::UndergroundUsIrregular)? > 0
            || board.get_forces_available(Forces::UsBase)? > 0),
        _ => panic!("Not implemented for {:?}", faction),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_definitions::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_wont_say_there_are_us_forces_available_if_there_are_none() -> Result<(), String> {
        let board = Board::new();

        assert!(!are_there_any_forces_of_a_faction_available(
            Factions::US,
            &board
        )?);

        Ok(())
    }

    #[test]
    fn test_will_say_there_are_us_forces_available_if_there_are() -> Result<(), String> {
        let mut board = Board::new();

        board.increase_forces_in_space(
            &Forces::UndergroundUsIrregular,
            SpaceIdentifiers::Available,
            1,
        )?;

        assert!(are_there_any_forces_of_a_faction_available(
            Factions::US,
            &board
        )?);

        Ok(())
    }
}
