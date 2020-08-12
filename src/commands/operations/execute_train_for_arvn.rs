use board::available_forces::AvailableForces;
use board::map::Map;
use board::space_identifiers::SpaceIdentifiers;
use board::track::Track;
use commands::atomic::deploy_arvn_troops_from_available::deploy_arvn_troops_from_available;
use commands::atomic::manipulate_arvn_resources::manipulate_arvn_resources;

pub fn execute_train_for_arvn(
    space_identifier: SpaceIdentifiers,
    number_of_troops: u8,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // For training in a location, ARVN spends 3 of its resources,
    // and then gets troops (6 as far as I know, likely changeable)
    // in the location specified.
    manipulate_arvn_resources(track, -3)?;
    deploy_arvn_troops_from_available(space_identifier, number_of_troops, map, available_forces)?;

    Ok(())
}
