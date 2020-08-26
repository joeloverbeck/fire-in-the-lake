use board::domain::board::Board;
use board::domain::queries::space_level_queries::get_spaces_where_nva_can_infiltrate_a_vc_base::get_spaces_where_nva_can_infiltrate_a_vc_base;

pub fn can_nva_infiltrate_any_vc_base_anywhere(board: &Board) -> Result<bool, String> {
    // -First replace VC bases in spaces with more NVA than VC pieces but no NVA Base already (Shifting Opposition, 4.4.1),
    // first tunneled bases, within that priority first in spaces with the highest population, finally in random spaces.
    // -If there are no NVA Bases available, they may first remove one base from the space in North Vietnam, Laos or cambodia
    // -With the most NVA pieces to Available in order to replace one VC Base.

    Ok(!get_spaces_where_nva_can_infiltrate_a_vc_base(board)
        .unwrap()
        .is_empty())
}

#[cfg(test)]

mod tests {

    use super::*;
    use game_definitions::forces::Forces;
    use game_definitions::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_nva_cannot_infiltrate_a_vc_base_in_a_space_where_it_has_a_base() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::VcBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::NvaTroop, 2, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundVcGuerrilla, 1, SpaceIdentifiers::Saigon)?;

        assert_eq!(can_nva_infiltrate_any_vc_base_anywhere(&board)?, false);

        Ok(())
    }

    #[test]
    fn test_nva_cannot_infiltrate_a_vc_base_in_a_space_where_it_has_fewer_numbers(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::VcBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::NvaTroop, 2, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundVcGuerrilla, 2, SpaceIdentifiers::Saigon)?;

        assert_eq!(can_nva_infiltrate_any_vc_base_anywhere(&board)?, false);

        Ok(())
    }

    #[test]
    fn test_nva_can_infiltrate_a_vc_base_in_a_space_where_it_has_greater_numbers_and_no_base(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::VcBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::NvaTroop, 2, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundNvaGuerrilla, 2, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundVcGuerrilla, 2, SpaceIdentifiers::Saigon)?;

        assert_eq!(can_nva_infiltrate_any_vc_base_anywhere(&board)?, true);

        Ok(())
    }
}
