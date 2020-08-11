use board::available_forces::AvailableForces;
use board::map::Map;
use board::space::Space;
use board::space_identifiers::SpaceIdentifiers;
use board::support::SupportLevels;
use board::track::Track;
use math::amount_that_was_removed_from_number::amount_that_was_removed_from_number;

pub fn deploy_nva_guerrillas_from_available(
    space_identifier: SpaceIdentifiers,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // Bases allow 1 underground guerrilla, and more equivalent to the level of the trail.
    // Even if there is Active Opposition in the location, if there is no base they can only place one guerrilla.
    let retrieved_space = map.get_space_mut(space_identifier).unwrap();

    // Let's see how this works.
    // Bases add 1 to possible guerrillas
    // Trail level adds as many as the level
    // However, if there's no base you can only add 1.
    if !retrieved_space.are_there_nva_bases() {
        // Can only add one guerrilla, and maybe if they are just at Active Opposition.
        if retrieved_space.get_support_level() == SupportLevels::ActiveOpposition {
            // Can only place one guerrilla.
            let amount_of_nva_guerrillas_available_removed: u8 =
                amount_that_was_removed_from_number(
                    available_forces.remove_amount_of_nva_guerrillas(1),
                );

            // They go into the space's forces as underground guerrillas.
            retrieved_space.set_number_of_underground_nva_guerrillas(
                retrieved_space.get_number_of_underground_nva_guerrillas()
                    + amount_of_nva_guerrillas_available_removed,
            );
        } else {
            todo!()
        }
    } else {
        // There are bases.
        let amount_of_nva_guerrillas_available_removed: u8 = amount_that_was_removed_from_number(
            available_forces.remove_amount_of_nva_guerrillas(1 + track.get_trail()),
        );

        // They go into the space's forces as underground guerrillas.
        retrieved_space.set_number_of_underground_nva_guerrillas(
            retrieved_space.get_number_of_underground_nva_guerrillas()
                + amount_of_nva_guerrillas_available_removed,
        );
    }

    Ok(())
}
