use board::map::Map;
use board::space::Space;
use board::space_identifiers::SpaceIdentifiers;

pub fn sweep(space_identifier: SpaceIdentifiers, map: &mut Map) -> Result<(), String> {
    let possible_retrieved_space = map.get_space_mut(space_identifier);

    if let Err(error) = possible_retrieved_space {
        return Err(format!(
            "Attempted a sweep at location '{:?}', but could not retrieve it! Error: {:?}",
            space_identifier, error
        ));
    } else if let Ok(retrieved_space) = possible_retrieved_space {
        // Activate all the VC guerrillas in the location
        let underground_vc_guerrillas_present =
            retrieved_space.get_number_of_underground_vc_guerrillas();
        retrieved_space.set_number_of_underground_vc_guerrillas(0);

        retrieved_space.set_number_of_active_vc_guerrillas(
            retrieved_space.get_number_of_active_vc_guerrillas()
                + underground_vc_guerrillas_present,
        );
    }

    Ok(())
}
