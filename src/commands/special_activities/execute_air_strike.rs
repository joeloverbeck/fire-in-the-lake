use board::available_forces::AvailableForces;
use board::map::Map;
use board::space::Space;
use board::space::Spaces;
use board::space_identifiers::SpaceIdentifiers;
use board::track::Track;

fn slaughter_active_vc_guerrillas(
    retrieved_space: &mut Spaces,
    available_forces: &mut AvailableForces,
) {
    let number_of_active_vc_guerrillas_in_space =
        retrieved_space.get_number_of_active_vc_guerrillas();
    retrieved_space.set_number_of_active_vc_guerrillas(0);
    available_forces.set_amount_of_vc_guerrillas(
        available_forces.get_amount_of_vc_guerrillas() + number_of_active_vc_guerrillas_in_space,
    );
}

pub fn execute_air_strike(
    air_strike_space: SpaceIdentifiers,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // Sends active guerrillas to the available forces.
    let retrieved_space = map.get_space_mut(air_strike_space).unwrap();

    slaughter_active_vc_guerrillas(retrieved_space, available_forces);

    // Shift space one level down.
    retrieved_space.shift_support_level_down();

    // Adjust VC victory marker.
    track.adjust_vc_victory_marker(map);

    // Degrade trail.
    track.set_trail(track.get_trail() - 1);

    Ok(())
}
