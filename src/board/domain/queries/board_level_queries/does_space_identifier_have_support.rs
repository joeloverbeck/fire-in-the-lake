use board::domain::board::Board;
use board::domain::queries::space_level_queries::does_space_have_support::does_space_have_support;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn does_space_identifier_have_support(
    space_identifier: SpaceIdentifiers,
    board: &Board,
) -> Result<bool, String> {
    let corresponding_space = board.get_space(space_identifier)?;

    Ok(does_space_have_support(&corresponding_space)?)
}

#[cfg(test)]
mod tests {

    use super::*;
    use board::domain::board::Board;
    use game_definitions::support_levels::SupportLevels;

    #[test]
    fn test_wont_say_space_has_support_if_it_doesnt_have() -> Result<(), String> {
        let mut board = Board::new();

        board.set_support_level_of_space(
            SupportLevels::PassiveOpposition,
            SpaceIdentifiers::PleikuDarlac,
        )?;

        assert!(!does_space_identifier_have_support(
            SpaceIdentifiers::PleikuDarlac,
            &board
        )?);

        Ok(())
    }

    #[test]
    fn test_will_say_space_has_support_if_it_does() -> Result<(), String> {
        let mut board = Board::new();

        board.set_support_level_of_space(
            SupportLevels::PassiveSupport,
            SpaceIdentifiers::PleikuDarlac,
        )?;

        assert!(does_space_identifier_have_support(
            SpaceIdentifiers::PleikuDarlac,
            &board
        )?);

        Ok(())
    }
}
