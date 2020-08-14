use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;
use decision_making::interpretation::operation_instructions::march_order::MarchOrder;
use decision_making::interpretation::transform_typed_digit_into_integer::transform_typed_digit_into_integer;
use decision_making::interpretation::transform_typed_space_into_space_identifier::transform_typed_space_into_space_identifier;

pub fn register_march_orders(
    split_typed_input_commands: Vec<&str>,
    interpreted_intentions: &mut InterpretedIntentions,
) {
    // Wants to register a march order.
    // Format: march_order:[troop_type]:[amount]:[from]:[to]
    interpreted_intentions.add_march_order(MarchOrder::new(
        split_typed_input_commands[1].to_string(),
        transform_typed_digit_into_integer(&split_typed_input_commands[2]),
        transform_typed_space_into_space_identifier(&split_typed_input_commands[3]),
        transform_typed_space_into_space_identifier(&split_typed_input_commands[4]),
    ));
}
