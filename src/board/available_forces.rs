use math::sum_i8_to_u8::sum_i8_to_u8;

pub struct AvailableForces {
    arvn_available: u8,
}

impl AvailableForces {
    pub fn new() -> AvailableForces {
        AvailableForces { arvn_available: 0 }
    }

    pub fn remove_amount_of_arvn_troops(&mut self, amount: u8) -> Result<u8, String> {
        // Will have to check edge cases, like not being able to remove the amount provided.
        let negative_amount: i8 = 0 - (amount as i8);

        // Will have to store the amount of arvn available before removing, because those will be all
        // that we will be able to add to the location, if that happens.
        let previous_amount_of_arvn_available = self.arvn_available;

        self.arvn_available = sum_i8_to_u8(negative_amount, self.arvn_available);

        Ok(i8::abs((previous_amount_of_arvn_available as i8) - (self.arvn_available as i8)) as u8)
    }

    pub fn set_amount_of_arvn_troops(&mut self, amount: u8) {
        self.arvn_available = amount;
    }
}

impl Default for AvailableForces {
    fn default() -> Self {
        Self::new()
    }
}
