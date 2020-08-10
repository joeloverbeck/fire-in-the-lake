use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use collections::extract_target_spaces_from_commands::extract_target_spaces_from_commands;
use commands::improve_trail_nva::improve_trail_nva;
use commands::operations::execute_rally_for_nva::execute_rally_for_nva;
use decision_making::input_commands::InputCommands;

pub fn execute_operation_for_nva(
    commands: Vec<InputCommands>,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
    _special_activity: bool,
) -> Result<(), String> {
    // In commands[1] we should have the name of the operation.
    // From then onwards, until a 'stop', should be the locations where it is performed
    if commands[1] == InputCommands::Rally {
        let command_locations = extract_target_spaces_from_commands(&commands, 2);

        execute_rally_for_nva(command_locations, map, track, available_forces)?;

        // As part of choosing rally, they get the opportunity to improve the trail
        // one level.
        let index_of_last_command = commands
            .iter()
            .position(|command| command == &InputCommands::Stop)
            .unwrap();

        if commands[index_of_last_command + 1] == InputCommands::Yes {
            // wants to improve the trail.
            improve_trail_nva(track)?;
        }
    } else {
        todo!()
    }

    Ok(())
}
