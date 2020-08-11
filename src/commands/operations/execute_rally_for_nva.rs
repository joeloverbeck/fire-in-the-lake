use board::available_forces::AvailableForces;
use board::map::Map;
use board::space_identifiers::SpaceIdentifiers;
use board::track::Track;
use commands::atomic::deploy_nva_guerrillas_from_available::deploy_nva_guerrillas_from_available;
use commands::manipulate_nva_resources::manipulate_nva_resources;

pub fn execute_rally_for_nva(
    spaces_for_operation: Vec<SpaceIdentifiers>,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // Pays a resource cost of 1 per space.
    let resources_to_pay: i8 = spaces_for_operation.len() as i8;

    manipulate_nva_resources(track, -resources_to_pay)?;

    for space in spaces_for_operation.iter() {
        deploy_nva_guerrillas_from_available(*space, map, track, available_forces)?;
    }

    Ok(())
}
