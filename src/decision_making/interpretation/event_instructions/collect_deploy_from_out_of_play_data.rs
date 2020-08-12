use decision_making::interpretation::event_instructions::deploy_from_out_of_play_data::DeployFromOutOfPlayData;
use decision_making::interpretation::event_instructions::form_typed_space::form_typed_space;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;
use decision_making::interpretation::transform_typed_digit_into_integer::transform_typed_digit_into_integer;
use decision_making::interpretation::transform_typed_space_into_space_identifier::transform_typed_space_into_space_identifier;

pub fn collect_deploy_from_out_of_play_data(
    split_typed_input_command: Vec<&str>,
    interpreted_intentions: &mut InterpretedIntentions,
) -> Result<(), String> {
    // According to the specifications for this type of instruction, the data comes as [unit_type]_[amount]_[space]
    let deploy_from_out_of_play_data: Vec<&str> = split_typed_input_command[2].split(':').collect();

    if deploy_from_out_of_play_data.len() != 3 {
        panic!("Had detected a instruction to deploy units from out of play, but the data wasn't correct: {:?}", deploy_from_out_of_play_data);
    }

    // All seems correct. Let's interpret this kind of special event data.
    let typed_space = form_typed_space(&deploy_from_out_of_play_data[2]);

    interpreted_intentions.add_to_deploy_from_out_of_play_data(DeployFromOutOfPlayData::new(
        deploy_from_out_of_play_data[0].to_string(),
        transform_typed_digit_into_integer(deploy_from_out_of_play_data[1]),
        transform_typed_space_into_space_identifier(&typed_space),
    ));

    Ok(())
}
