pub fn extract_multiword_command(multiword_command: &str) -> Vec<String> {
    let operation_command_iter = multiword_command.split_whitespace();

    let mut operation_command: Vec<String> = Vec::new();

    for item in operation_command_iter {
        operation_command.push(item.to_string());
    }

    operation_command
}
