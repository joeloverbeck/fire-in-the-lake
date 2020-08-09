use board::available_forces::AvailableForces;
use board::map::Map;
use board::space::Space;
use board::space_identifiers::translate_space_name_to_identifier;

pub struct DeployArvnTroopsFromAvailable {}

impl Default for DeployArvnTroopsFromAvailable {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployArvnTroopsFromAvailable {
    pub fn new() -> DeployArvnTroopsFromAvailable {
        DeployArvnTroopsFromAvailable {}
    }

    pub fn execute(
        &self,
        location: &str,
        number_of_troops: u8,
        map: &mut Map,
        available_forces: &mut AvailableForces,
    ) -> Result<(), String> {
        let space_identifier = translate_space_name_to_identifier(&String::from(location));

        let mut amount_of_arvn_available_removed: u8 = 0;

        if let Ok(amount_removed) = available_forces.remove_amount_of_arvn_troops(number_of_troops)
        {
            amount_of_arvn_available_removed = amount_removed;
        }

        let retrieved_space = map.get_space_mut(space_identifier);

        if let Ok(space) = retrieved_space {
            space.set_number_of_arvn_troops(
                space.get_number_of_arvn_troops() + amount_of_arvn_available_removed,
            )?;
        }

        Ok(())
    }
}
