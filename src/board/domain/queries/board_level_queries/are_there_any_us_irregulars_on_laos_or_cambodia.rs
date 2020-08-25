use board::domain::queries::board_level_queries::is_there_any_number_of_a_specific_force_in_a_geographic_area::is_there_any_number_of_a_specific_force_in_a_geographic_area;
use board::domain::board::Board;
use game_definitions::forces::Forces;
use game_definitions::geographic_areas::GeographicAreas;

pub fn are_there_any_us_irregulars_on_laos_or_cambodia(board: &Board) -> Result<bool, String> {
    Ok(
        (is_there_any_number_of_a_specific_force_in_a_geographic_area(
            Forces::UndergroundUsIrregular,
            &GeographicAreas::Laos,
            board,
        )? || is_there_any_number_of_a_specific_force_in_a_geographic_area(
            Forces::ActiveUsIrregular,
            &GeographicAreas::Laos,
            board,
        )?) || (is_there_any_number_of_a_specific_force_in_a_geographic_area(
            Forces::UndergroundUsIrregular,
            &GeographicAreas::Cambodia,
            board,
        )? || is_there_any_number_of_a_specific_force_in_a_geographic_area(
            Forces::ActiveUsIrregular,
            &GeographicAreas::Cambodia,
            board,
        )?),
    )
}

#[cfg(test)]

mod tests {
    use super::*;
    use game_definitions::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_wont_say_there_are_us_irregulars_in_cambodia_if_there_arent_any() -> Result<(), String>
    {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ActiveUsIrregular, 1, SpaceIdentifiers::PleikuDarlac)?;

        assert!(!are_there_any_us_irregulars_on_laos_or_cambodia(&board)?);

        Ok(())
    }

    #[test]
    fn test_will_say_there_are_us_irregulars_in_cambodia_if_there_are() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(
            Forces::ActiveUsIrregular,
            1,
            SpaceIdentifiers::TheParrotsBeak,
        )?;

        assert!(are_there_any_us_irregulars_on_laos_or_cambodia(&board)?);

        Ok(())
    }

    #[test]
    fn test_will_say_there_are_us_irregulars_in_laos_if_there_are() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ActiveUsIrregular, 1, SpaceIdentifiers::CentralLaos)?;

        assert!(are_there_any_us_irregulars_on_laos_or_cambodia(&board)?);

        Ok(())
    }
}
