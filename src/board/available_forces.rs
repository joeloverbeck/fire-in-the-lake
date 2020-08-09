use math::produce_absolute_difference_between_numbers::produce_absolute_difference_between_numbers;
use math::produce_negative_number_from_u8::produce_negative_number_from_u8;
use math::sum_i8_to_u8::sum_i8_to_u8;

pub struct AvailableForces {
    arvn_available: u8,
    nva_guerrillas_available: u8,
}

impl AvailableForces {
    pub fn new() -> AvailableForces {
        AvailableForces {
            arvn_available: 0,
            nva_guerrillas_available: 0,
        }
    }

    pub fn remove_amount_of_arvn_troops(&mut self, amount: u8) -> Result<u8, String> {
        // Will have to check edge cases, like not being able to remove the amount provided.
        let negative_amount: i8 = produce_negative_number_from_u8(amount);

        // Will have to store the amount of arvn available before removing, because those will be all
        // that we will be able to add to the location, if that happens.
        let previous_amount_of_arvn_available = self.arvn_available;

        self.arvn_available = sum_i8_to_u8(negative_amount, self.arvn_available);

        Ok(produce_absolute_difference_between_numbers(
            previous_amount_of_arvn_available,
            self.arvn_available,
        ))
    }

    pub fn set_amount_of_arvn_troops(&mut self, amount: u8) {
        self.arvn_available = amount;
    }

    pub fn set_amount_of_nva_guerrillas(&mut self, amount: u8) {
        self.nva_guerrillas_available = amount;
    }

    pub fn remove_amount_of_nva_guerrillas(&mut self, amount: u8) -> Result<u8, String> {
        // Will have to check edge cases, like not being able to remove the amount provided.
        let negative_amount: i8 = produce_negative_number_from_u8(amount);

        // Will have to store the amount of nva guerrillas available before removing, because those will be all
        // that we will be able to add to the location, if that happens.
        let previous_amount_of_nva_guerrillas_available = self.nva_guerrillas_available;

        self.nva_guerrillas_available =
            sum_i8_to_u8(negative_amount, self.nva_guerrillas_available);

        Ok(produce_absolute_difference_between_numbers(
            previous_amount_of_nva_guerrillas_available,
            self.nva_guerrillas_available,
        ))
    }
}

impl Default for AvailableForces {
    fn default() -> Self {
        Self::new()
    }
}
