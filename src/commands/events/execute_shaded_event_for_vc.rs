use board::map::Map;
use board::space_identifiers::SpaceIdentifiers;
use board::track::Track;
use commands::manipulate_aid::manipulate_aid;
use commands::shift_support_of_space::shift_support_of_space;
use decision_making::input_commands::InputCommands;

pub fn execute_shaded_event_for_vc(
    card_number: u8,
    _commands: Vec<InputCommands>,
    map: &mut Map,
    track: &mut Track,
) -> Result<(), String> {
    // Depending on the card (one of many, many, many), well execute one thing or another.
    match card_number {
        107 => {
            // Shift down support level in Saigon. Adjust Victory Markers. Lower aid.
            shift_support_of_space(map.get_space_mut(SpaceIdentifiers::Saigon).unwrap(), -1)?;
            manipulate_aid(track, -12)?;
            track.adjust_us_victory_marker(&map);

            Ok(())
        }
        _ => Ok(()),
    }
}
