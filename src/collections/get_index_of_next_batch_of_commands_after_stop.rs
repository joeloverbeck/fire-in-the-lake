use decision_making::input_commands::InputCommands;

pub fn get_index_of_next_batch_of_commands_after_stop(commands: &[InputCommands]) -> usize {
    let index_of_next_stop = commands
        .iter()
        .position(|command| command == &InputCommands::Stop)
        .unwrap();

    index_of_next_stop + 1
}
