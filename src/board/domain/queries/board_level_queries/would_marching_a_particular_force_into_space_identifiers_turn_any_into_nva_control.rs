use board::domain::queries::board_level_queries::would_marching_a_particular_force_into_a_space_turn_it_into_nva_control::would_marching_a_particular_force_into_a_space_turn_it_into_nva_control;
use board::domain::board::Board;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::forces::Forces;

pub fn would_marching_a_particular_force_into_space_identifiers_turn_any_into_nva_control(
    force: Forces,
    space_identifiers: Vec<&SpaceIdentifiers>,
    board: &Board,
) -> Result<bool, String> {
    let mut would_marching_turn_space_into_nva_control = false;

    // This is a complicated one. For *all* spaces on the board, must look at which have any of the space identifiers
    // as adjacent. Then, in the checked space, must see how many of the passed force they have. Then we need to check if
    // adding those to the space corresponding to the space identifier would turn it into NvaControl.
    for (_, occupable_space) in board.get_occupable_spaces()?.iter() {
        would_marching_turn_space_into_nva_control =
            would_marching_a_particular_force_into_a_space_turn_it_into_nva_control(
                force,
                occupable_space,
                &space_identifiers,
                board,
            )?;

        if would_marching_turn_space_into_nva_control {
            break;
        }
    }

    Ok(would_marching_turn_space_into_nva_control)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_marching_nva_troops_into_a_space_identifier_would_turn_it_into_nva_control(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 4, SpaceIdentifiers::BinhTuyBinhThuan)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundArvnRanger, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundVcGuerrilla, 1, SpaceIdentifiers::Saigon)?;

        assert!(
            would_marching_a_particular_force_into_space_identifiers_turn_any_into_nva_control(
                Forces::NvaTroop,
                vec![&SpaceIdentifiers::Saigon],
                &board
            )?
        );

        Ok(())
    }

    #[test]
    fn test_marching_too_few_nva_troops_into_a_space_identifier_wouldnt_turn_it_into_nva_control(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::BinhTuyBinhThuan)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundArvnRanger, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundVcGuerrilla, 1, SpaceIdentifiers::Saigon)?;

        assert_eq!(
            would_marching_a_particular_force_into_space_identifiers_turn_any_into_nva_control(
                Forces::NvaTroop,
                vec![&SpaceIdentifiers::Saigon],
                &board
            )?,
            false
        );

        Ok(())
    }
}
