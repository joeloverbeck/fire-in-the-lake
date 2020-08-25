use board::domain::board::Board;
use board::domain::queries::space_level_queries::are_there_any_of_a_particular_force_in_space::are_there_any_of_a_particular_force_in_space;
use board::domain::queries::space_level_queries::will_moving_in_number_of_nva_forces_turn_space_into_nva_control::will_moving_in_number_of_nva_forces_turn_space_into_nva_control;
use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::control_types::ControlTypes;
use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn would_marching_a_particular_force_into_a_space_turn_it_into_nva_control(
    force: Forces,
    space: &Spaces,
    possible_adjacent: &[&SpaceIdentifiers],
    board: &Board,
) -> Result<bool, String> {
    let mut would_marching_turn_space_into_nva_control = false;

    for &space_identifier in possible_adjacent {
        if !space.is_adjacent_to_space(*space_identifier)? {
            continue;
        }

        // The passed space_identifier is adjacent to the current occupable space
        // we are looking at in the board. We need to check if there are forces of the
        // needed type to march into the passed space_identifier.
        if !are_there_any_of_a_particular_force_in_space(force, &space)? {
            continue;
        }

        let number_of_particular_force_in_space = space.get_forces(force)?;

        // There are forces of the needed type. We need another check, though: if the space_identifier
        // is already NvaControl, it would obviously not change to NvaControl
        let corresponding_destination_space = board.get_space(*space_identifier)?;

        if corresponding_destination_space.get_control_type()? == &ControlTypes::Nva {
            continue;
        }

        // Final check:
        if will_moving_in_number_of_nva_forces_turn_space_into_nva_control(
            number_of_particular_force_in_space,
            corresponding_destination_space,
        )? {
            would_marching_turn_space_into_nva_control = true;
            break;
        }
    }

    Ok(would_marching_turn_space_into_nva_control)
}
