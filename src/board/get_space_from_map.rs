use board::map::Map;
use board::space::Spaces;
use board::space_identifiers::SpaceIdentifiers;

pub fn get_space_from_map(
    space_identifier: SpaceIdentifiers,
    map: &mut Map,
) -> Result<&mut Spaces, String> {
    Ok(map.get_space_mut(space_identifier).unwrap())
}
