use board::map::Map;
use board::space::Space;
use board::space_identifiers::translate_space_name_to_identifier;
use board::support::SupportLevels;
use board::track::Track;

pub fn set_space_to_active_support(
    location: &str,
    map: &mut Map,
    track: &mut Track,
) -> Result<(), String> {
    let space_identifier = translate_space_name_to_identifier(location);

    let retrieved_space = map.get_space_mut(space_identifier).unwrap();

    retrieved_space.set_support_level(SupportLevels::ActiveSupport);

    // Should trigger adjusting the victory marker for the us.
    track.adjust_us_victory_marker(map);

    Ok(())
}
