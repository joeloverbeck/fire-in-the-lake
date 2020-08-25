use board::domain::board::Board;
use board::domain::queries::space_level_queries::calculate_number_of_faction_cubes_in_space::calculate_number_of_faction_cubes_in_space;
use game_definitions::factions::Factions;

pub fn are_there_any_cubes_of_a_faction_anywhere(
    faction: Factions,
    board: &Board,
) -> Result<bool, String> {
    Ok(board
        .get_occupable_spaces()?
        .iter()
        .map(|(_, occupable_space)| {
            calculate_number_of_faction_cubes_in_space(faction, occupable_space).unwrap()
        })
        .sum::<u8>()
        > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_definitions::forces::Forces;
    use game_definitions::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_will_say_there_are_cubes_of_a_faction_somewhere_if_there_are_indeed_any_cubes_of_that_faction(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::PleikuDarlac)?;

        assert!(are_there_any_cubes_of_a_faction_anywhere(
            Factions::US,
            &board
        )?);

        Ok(())
    }

    #[test]
    fn test_wont_say_there_are_cubes_of_a_specific_faction_if_the_forces_present_arent_cubes(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(
            Forces::UndergroundUsIrregular,
            1,
            SpaceIdentifiers::PleikuDarlac,
        )?;

        assert!(!are_there_any_cubes_of_a_faction_anywhere(
            Factions::US,
            &board
        )?);

        Ok(())
    }
}
