use board::map::Map;
use board::space::Spaces;
use board::space_identifiers::translate_space_name_to_identifier;
use decision_making::input_commands::InputCommands;

pub fn get_space_from_map(location: InputCommands, map: &mut Map) -> Result<&mut Spaces, String> {
    Ok(map
        .get_space_mut(translate_space_name_to_identifier(location))
        .unwrap())
}
