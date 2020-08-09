use board::map::Map;
use board::space::Space;
use board::space_identifiers::translate_space_name_to_identifier;

pub struct Sweep {}

impl Default for Sweep {
    fn default() -> Self {
        Self::new()
    }
}

impl Sweep {
    pub fn new() -> Sweep {
        Sweep {}
    }

    pub fn execute(&self, location: &str, map: &mut Map) -> Result<(), String> {
        let space_identifier = translate_space_name_to_identifier(&String::from(location));

        let possible_retrieved_space = map.get_space_mut(space_identifier);

        if let Err(error) = possible_retrieved_space {
            return Err(format!(
                "Attempted a sweep at location '{:?}', but could not retrieve it! Error: {:?}",
                location, error
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
}
