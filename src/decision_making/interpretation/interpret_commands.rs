use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;
use decision_making::interpretation::transform_typed_digit_into_integer::transform_typed_digit_into_integer;
use decision_making::interpretation::transform_typed_space_into_space_identifier::transform_typed_space_into_space_identifier;

fn does_typed_command_refer_to_space(typed_input_command: &str) -> bool {
    typed_input_command == "north vietnam"
        || typed_input_command == "saigon"
        || typed_input_command == "an loc"
        || typed_input_command == "can tho"
        || typed_input_command == "the parrot's beak"
        || typed_input_command == "kien phong"
        || typed_input_command == "kien giang"
        || typed_input_command == "quang tri"
        || typed_input_command == "binh dinh"
        || typed_input_command == "pleiku"
        || typed_input_command == "hue"
        || typed_input_command == "quang tin"
        || typed_input_command == "quang duc"
        || typed_input_command == "binh tuy"
}

fn is_the_typed_command_a_number(typed_input_command: &str) -> bool {
    typed_input_command == "6"
}

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

        if typed_input_command == "operation" {
            interpreted_intentions.wants_to_do_an_operation();
        } else if typed_input_command == "pass" {
            interpreted_intentions.wants_to_pass();
        } else if typed_input_command == "event" {
            interpreted_intentions.wants_to_activate_the_event();
        } else if typed_input_command == "op only" || typed_input_command == "operation only" {
            interpreted_intentions.wants_to_do_an_operation_only();
        } else if typed_input_command == "train" {
            interpreted_intentions.wants_to_train();
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
        } else {
            panic!(
                "Not implemented for {:?}. Interpreted intentions: {:?}",
                typed_input_command, interpreted_intentions
            );
        }
    }

    Ok(interpreted_intentions)
}