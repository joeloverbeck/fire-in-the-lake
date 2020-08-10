use decision_making::input_commands::InputCommands;

pub fn translate_input_command_to_digit(input_command: InputCommands) -> u8 {
    match input_command {
        InputCommands::Six => 6,
        _ => panic!("{:?}", input_command),
    }
}
