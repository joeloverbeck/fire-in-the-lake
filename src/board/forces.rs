#[derive(Debug)]
pub struct Forces {
    arvn_troops: u8,
}

impl Default for Forces {
    fn default() -> Self {
        Self::new()
    }
}

impl Forces {
    pub fn new() -> Forces {
        Forces { arvn_troops: 0 }
    }

    pub fn get_number_of_arvn_troops(&self) -> u8 {
        self.arvn_troops
    }

    pub fn set_number_of_arvn_troops(
        &mut self,
        new_number_of_arvn_troops: u8,
    ) -> Result<(), String> {
        self.arvn_troops = new_number_of_arvn_troops;

        Ok(())
    }
}
