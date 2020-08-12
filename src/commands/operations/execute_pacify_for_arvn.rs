use board::map::Map;
use board::space_identifiers::SpaceIdentifiers;
use board::track::Track;
use commands::atomic::shift_support_of_space::shift_support_of_space;

pub fn execute_pacify_for_arvn(
    space_identifier: SpaceIdentifiers,
    map: &mut Map,
    track: &mut Track,
) -> Result<(), String> {
    // It costs three resources.
    track.set_arvn_resources(track.get_arvn_resources() - 3);

    // Shift support upwards for the location.
    let retrieved_space = map.get_space_mut(space_identifier).unwrap();

    shift_support_of_space(retrieved_space, 1)?;

    // Trigger US' victory marker calculation.
    track.adjust_us_victory_marker(map);

    Ok(())
}
