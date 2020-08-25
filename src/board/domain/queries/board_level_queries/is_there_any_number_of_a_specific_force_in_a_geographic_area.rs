use board::domain::board::Board;
use board::domain::space::Space;
use game_definitions::forces::Forces;
use game_definitions::geographic_areas::GeographicAreas;

pub fn is_there_any_number_of_a_specific_force_in_a_geographic_area(
    force: Forces,
    geographic_area: &GeographicAreas,
    board: &Board,
) -> Result<bool, String> {
    // We go through all the stored spaces, and if any of them are in the geographic area, there just needs to hold
    // one of the passed force
    Ok(board
        .get_occupable_spaces()?
        .iter()
        .filter(|(_, occupable_space)| {
            occupable_space.get_geographic_area().unwrap() == geographic_area
        })
        .any(|(_, occupable_space)| occupable_space.get_forces(force).unwrap() > 0))
}

#[cfg(test)]

mod tests {

    use super::*;
    use game_definitions::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_can_figure_out_if_there_isnt_any_of_a_particular_force_in_a_geographic_area(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::TheFishhook)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::TheParrotsBeak)?;
        board.set_forces_in_space(
            Forces::UndergroundUsIrregular,
            1,
            SpaceIdentifiers::CentralLaos,
        )?;

        assert_eq!(
            is_there_any_number_of_a_specific_force_in_a_geographic_area(
                Forces::UndergroundUsIrregular,
                &GeographicAreas::NorthVietnam,
                &board
            )?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_can_figure_out_if_there_are_any_of_a_particular_force_in_a_geographic_area(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::TheFishhook)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::TheParrotsBeak)?;
        board.set_forces_in_space(
            Forces::UndergroundUsIrregular,
            1,
            SpaceIdentifiers::CentralLaos,
        )?;

        assert!(
            is_there_any_number_of_a_specific_force_in_a_geographic_area(
                Forces::UndergroundUsIrregular,
                &GeographicAreas::Laos,
                &board
            )?
        );

        Ok(())
    }
}
