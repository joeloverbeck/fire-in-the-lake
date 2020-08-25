use board::domain::board::Board;
use board::domain::space::Space;
use game_definitions::control_types::ControlTypes;
use game_definitions::geographic_areas::GeographicAreas;

pub fn is_any_space_at_a_specific_control_type_in_a_geographic_area(
    control_type: &ControlTypes,
    geographic_area: &GeographicAreas,
    board: &Board,
) -> Result<bool, String> {
    Ok(board
        .get_occupable_spaces()?
        .iter()
        .any(|(_, occupable_space)| {
            occupable_space.get_geographic_area().unwrap() == geographic_area
                && occupable_space.get_control_type().unwrap() == control_type
        }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_definitions::space_identifiers::SpaceIdentifiers;

    #[test]
    fn test_an_occupable_space_in_a_geographical_area_without_a_certain_control_returns_false(
    ) -> Result<(), String> {
        let board = Board::new();

        assert_eq!(
            is_any_space_at_a_specific_control_type_in_a_geographic_area(
                &ControlTypes::Nva,
                &GeographicAreas::NorthVietnam,
                &board
            )?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_an_occupable_space_in_a_geographical_area_with_a_certain_control_returns_true(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_control_type_of_space(ControlTypes::Nva, SpaceIdentifiers::TheParrotsBeak)?;

        assert_eq!(
            is_any_space_at_a_specific_control_type_in_a_geographic_area(
                &ControlTypes::Nva,
                &GeographicAreas::Cambodia,
                &board
            )?,
            true
        );

        Ok(())
    }
}
