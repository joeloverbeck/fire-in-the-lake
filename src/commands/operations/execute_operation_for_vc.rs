use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use collections::extract_target_spaces_from_commands::extract_target_spaces_from_commands;
use commands::operations::execute_rally_for_vc::execute_rally_for_vc;
use decision_making::input_commands::InputCommands;

pub fn execute_operation_for_vc(
    commands: Vec<InputCommands>,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    if commands[0] == InputCommands::Rally {
        let command_locations = extract_target_spaces_from_commands(&commands, 1);

        execute_rally_for_vc(command_locations, map, track, available_forces)?;
    } else {
        todo!()
    }

    Ok(())
}
