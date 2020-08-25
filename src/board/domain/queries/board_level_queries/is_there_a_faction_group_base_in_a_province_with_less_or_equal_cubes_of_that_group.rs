use board::domain::board::Board;
use board::domain::queries::space_level_queries::calculate_number_of_coin_cubes_in_space::calculate_number_of_coin_cubes_in_space;
use board::domain::queries::space_level_queries::calculate_number_of_faction_group_bases_in_space::calculate_number_of_faction_group_bases_in_space;
use game_definitions::faction_groups::FactionGroups;

pub fn is_there_a_faction_group_base_in_a_province_with_less_or_equal_cubes_of_that_group(
    faction_group: &FactionGroups,
    max_number_of_cubes: u8,
    board: &Board,
) -> Result<bool, String> {
    Ok(board
        .get_occupable_spaces()?
        .iter()
        .any(|(_, occupable_space)| {
            // If faction group is Coin, those cubes would be UsTroops, ArvnTroops or ArvnPolice.
            // If faction group is Insurgent, those cubes would just be NvaTroops
            if faction_group == &FactionGroups::Coin {
                if calculate_number_of_faction_group_bases_in_space(
                    FactionGroups::Coin,
                    &occupable_space,
                )
                .unwrap()
                    > 0
                    && calculate_number_of_coin_cubes_in_space(&occupable_space).unwrap()
                        <= max_number_of_cubes
                {
                    return true;
                }

                false
            } else {
                panic!("Case not handled when the faction group is not Coin.");
            }
        }))
}

#[cfg(test)]

mod tests {
    use super::*;
    use game_definitions::forces::Forces;
    use game_definitions::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_query_to_whether_theres_a_coin_base_in_a_space_with_less_than_a_number_fails_if_theres_a_higher_number(
    ) -> Result<(), String> {
        // Regarding the original event: It's only effective in its unshaded part if there's a COIN base in a province with less or equal
        // COIN "cubes" than a number specifically (US to Casualties).
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::ArvnPolice, 2, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::Saigon)?;

        assert_eq!(
            is_there_a_faction_group_base_in_a_province_with_less_or_equal_cubes_of_that_group(
                &FactionGroups::Coin,
                2,
                &board
            )?,
            false
        );

        Ok(())
    }
}
