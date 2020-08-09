#[derive(Debug)]
pub struct Forces {
    arvn_troops: u8,
    nva_bases: u8,
    underground_nva_guerrillas: u8,
}

impl Default for Forces {
    fn default() -> Self {
        Self::new()
    }
}

impl Forces {
    pub fn new() -> Forces {
        Forces {
            arvn_troops: 0,
            nva_bases: 0,
            underground_nva_guerrillas: 0,
        }
    }

    pub fn get_number_of_arvn_troops(&self) -> u8 {
        self.arvn_troops
    }

    pub fn get_number_of_nva_bases(&self) -> u8 {
        self.nva_bases
    }

    pub fn set_number_of_nva_bases(&mut self, new_number_of_nva_bases: u8) {
        self.nva_bases = new_number_of_nva_bases
    }

    pub fn set_number_of_arvn_troops(
        &mut self,
        new_number_of_arvn_troops: u8,
    ) -> Result<(), String> {
        self.arvn_troops = new_number_of_arvn_troops;

        Ok(())
    }

    pub fn get_number_of_underground_nva_guerrillas(&self) -> u8 {
        self.underground_nva_guerrillas
    }

    pub fn set_number_of_underground_nva_guerrillas(
        &mut self,
        new_number_of_underground_nva_guerrillas: u8,
    ) {
        self.underground_nva_guerrillas = new_number_of_underground_nva_guerrillas;
    }
}
