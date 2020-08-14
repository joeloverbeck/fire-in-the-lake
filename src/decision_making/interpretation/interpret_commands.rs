use decision_making::interpretation::does_typed_command_refer_to_space::does_typed_command_refer_to_space;
use decision_making::interpretation::event_instructions::collect_deploy_from_out_of_play_data::collect_deploy_from_out_of_play_data;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;
use decision_making::interpretation::is_the_typed_command_a_number::is_the_typed_command_a_number;
use decision_making::interpretation::operation_instructions::register_infiltrate_instructions::register_infiltrate_instructions;
use decision_making::interpretation::operation_instructions::register_march_orders::register_march_orders;
use decision_making::interpretation::transform_typed_digit_into_integer::transform_typed_digit_into_integer;
use decision_making::interpretation::transform_typed_space_into_space_identifier::transform_typed_space_into_space_identifier;

pub fn interpret_commands(
    typed_input_commands: Vec<String>,
) -> Result<InterpretedIntentions, String> {
    let mut interpreted_intentions = InterpretedIntentions::new();

    // Should go through the commands and figure what the player wants to do with
    // his turn, depending on the words in the array as well as their order.
    for typed_input_command in typed_input_commands.iter() {
        println!(
            "Current typed input command looking at: {:?}",
            typed_input_command
        );

        // The most complicated input command to deal with is a variation on one
        // that the player doesn't even type directly: they are instructions for
        // the events to play.
        let split_typed_input_command: Vec<&str> = typed_input_command.split(' ').collect();
        let split_with_colon_typed_input_command: Vec<&str> =
            typed_input_command.split(':').collect();

        if split_typed_input_command[0] == "event_instruction" {
            // The second item in that array will be the specific type of instruction. Very different ones.
            if split_typed_input_command[1] == "deploy_from_out_of_play" {
                collect_deploy_from_out_of_play_data(
                    split_typed_input_command,
                    &mut interpreted_intentions,
                )?;
            } else {
                todo!();
            }
        } else if split_with_colon_typed_input_command[0] == "march_order" {
            register_march_orders(
                split_with_colon_typed_input_command,
                &mut interpreted_intentions,
            );
        } else if split_with_colon_typed_input_command[0] == "infiltrate_instructions" {
            register_infiltrate_instructions(
                split_with_colon_typed_input_command,
                &mut interpreted_intentions,
            )?;
        } else if typed_input_command == "operation" {
            interpreted_intentions.wants_to_do_an_operation();
        } else if typed_input_command == "pass" {
            interpreted_intentions.wants_to_pass();
        } else if typed_input_command == "event" {
            interpreted_intentions.wants_to_activate_the_event();
        } else if typed_input_command == "op only" || typed_input_command == "operation only" {
            interpreted_intentions.wants_to_do_an_operation_only();
        } else if typed_input_command == "train" {
            interpreted_intentions.wants_to_train();
        } else if typed_input_command == "march" {
            interpreted_intentions.wants_to_march();
        } else if typed_input_command == "rally" {
            interpreted_intentions.wants_to_rally();
        } else if typed_input_command == "sweep" {
            interpreted_intentions.wants_to_sweep();
        } else if does_typed_command_refer_to_space(typed_input_command)
            && interpreted_intentions.has_it_decided_on_an_operation()
            && !interpreted_intentions.has_it_chosen_a_special_activity()
        {
            interpreted_intentions.add_space_for_operation(
                transform_typed_space_into_space_identifier(&typed_input_command),
            );
        } else if does_typed_command_refer_to_space(typed_input_command)
            && interpreted_intentions.does_it_want_to_activate_the_event()
        {
            interpreted_intentions.add_space_for_event(
                transform_typed_space_into_space_identifier(&typed_input_command),
            );
        } else if does_typed_command_refer_to_space(typed_input_command)
            && interpreted_intentions.has_it_decided_on_an_operation()
            && interpreted_intentions.has_it_chosen_a_special_activity()
        {
            interpreted_intentions.add_space_for_special_activity(
                transform_typed_space_into_space_identifier(&typed_input_command),
            );
        } else if is_the_typed_command_a_number(typed_input_command)
            && interpreted_intentions.does_it_want_to_train()
        {
            interpreted_intentions
                .add_digit_for_operation(transform_typed_digit_into_integer(typed_input_command));
        } else if typed_input_command == "pacify" {
            interpreted_intentions.wants_to_pacify();
        } else if typed_input_command == "improve trail" {
            interpreted_intentions.wants_to_improve_the_trail();
        } else if typed_input_command == "govern" {
            interpreted_intentions.wants_to_govern();
        } else if typed_input_command == "tax" {
            interpreted_intentions.wants_to_tax();
        } else if typed_input_command == "infiltrate" {
            interpreted_intentions.wants_to_infiltrate();
        } else {
            panic!(
                "Not implemented for {:?}. Interpreted intentions: {:?}",
                typed_input_command, interpreted_intentions
            );
        }
    }

    Ok(interpreted_intentions)
}
