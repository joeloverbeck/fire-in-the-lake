use board::domain::board::Board;
use game_definitions::forces::Forces;

pub fn are_there_any_casualties(board: &Board) -> Result<bool, String> {
    // NOTE: this works under the assumption that only US forces go to Casualties.

    Ok(
        board.get_forces_in_casualties(Forces::UndergroundUsIrregular)? > 0
            || board.get_forces_in_casualties(Forces::ActiveUsIrregular)? > 0
            || board.get_forces_in_casualties(Forces::UsTroop)? > 0
            || board.get_forces_in_casualties(Forces::UsBase)? > 0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_definitions::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_wont_say_there_are_casualties_if_there_are_none() -> Result<(), String> {
        let board = Board::new();

        assert!(!are_there_any_casualties(&board)?);

        Ok(())
    }

    #[test]
    fn test_will_say_there_are_casualties_if_there_are() -> Result<(), String> {
        let mut board = Board::new();

        board.increase_forces_in_space(&Forces::UsTroop, SpaceIdentifiers::Casualties, 1)?;

        assert!(are_there_any_casualties(&board)?);

        Ok(())
    }
}
