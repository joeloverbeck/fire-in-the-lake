use board::map::Map;
use board::space_identifiers::translate_space_name_to_identifier;
use board::track::Track;
use commands::shift_support_of_space::shift_support_of_space;
use decision_making::input_commands::InputCommands;

pub fn execute_pacify_for_arvn(
    location: InputCommands,
    map: &mut Map,
    track: &mut Track,
) -> Result<(), String> {
    // It costs three resources.
    track.set_arvn_resources(track.get_arvn_resources() - 3);

    // Shift support upwards for the location.
    let space_identifier = translate_space_name_to_identifier(location);
    let retrieved_space = map.get_space_mut(space_identifier).unwrap();

    shift_support_of_space(retrieved_space, 1)?;

    // Trigger US' victory marker calculation.
    track.adjust_us_victory_marker(map);

    Ok(())
}
