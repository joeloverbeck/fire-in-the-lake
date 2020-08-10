use decision_making::input_commands::InputCommands;

pub fn extract_target_spaces_from_commands(
    commands: &[InputCommands],
    index_where_spaces_start: usize,
) -> Vec<InputCommands> {
    let mut command_locations: Vec<InputCommands> = Vec::new();

    for item in commands.iter().skip(index_where_spaces_start) {
        if item == &InputCommands::Stop {
            break;
        }

        command_locations.push(*item);
    }

    command_locations
}
