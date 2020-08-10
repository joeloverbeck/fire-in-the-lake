use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::atomic::deploy_nva_guerrillas_from_available::deploy_nva_guerrillas_from_available;
use commands::manipulate_nva_resources::manipulate_nva_resources;
use decision_making::input_commands::InputCommands;

pub fn execute_rally_for_nva(
    locations: Vec<InputCommands>,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // Pays a resource cost of 1 per space.
    let resources_to_pay: i8 = locations.len() as i8;

    manipulate_nva_resources(track, -resources_to_pay)?;

    for location in locations.iter() {
        deploy_nva_guerrillas_from_available(*location, map, track, available_forces)?;
    }

    Ok(())
}
