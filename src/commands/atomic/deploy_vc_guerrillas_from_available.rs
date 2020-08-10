use board::available_forces::AvailableForces;
use board::get_space_from_map::get_space_from_map;
use board::map::Map;
use board::space::Space;
use board::track::Track;
use decision_making::input_commands::InputCommands;
use math::amount_that_was_removed_from_number::amount_that_was_removed_from_number;

pub fn deploy_vc_guerrillas_from_available(
    location: InputCommands,
    map: &mut Map,
    _track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // Bases allow 1 underground guerrilla, and more equivalent to the population level (NOT THE TRAIL, as in the case of the NVA).
    // Even if there is Active Opposition in the location, if there is no base they can only place one guerrilla.
    let retrieved_space = get_space_from_map(location, map)?;

    // Let's see how this works.
    // Bases add 1 to possible guerrillas
    // Population value add as many as the population value.
    // However, if there's no base you can only add 1.
    if !retrieved_space.are_there_vc_bases() {
        // Can only place one guerrilla.
        let amount_of_vc_guerrillas_available_removed: u8 =
            amount_that_was_removed_from_number(available_forces.remove_amount_of_vc_guerrillas(1));

        // They go into the space's forces as underground guerrillas.
        retrieved_space.set_number_of_underground_vc_guerrillas(
            retrieved_space.get_number_of_underground_vc_guerrillas()
                + amount_of_vc_guerrillas_available_removed,
        );
    } else {
        // There are bases.
        let amount_of_vc_guerrillas_available_removed: u8 = amount_that_was_removed_from_number(
            available_forces
                .remove_amount_of_vc_guerrillas(1 + retrieved_space.get_population_value()),
        );

        // They go into the space's forces as underground guerrillas.
        retrieved_space.set_number_of_underground_vc_guerrillas(
            retrieved_space.get_number_of_underground_vc_guerrillas()
                + amount_of_vc_guerrillas_available_removed,
        );
    }

    Ok(())
}
