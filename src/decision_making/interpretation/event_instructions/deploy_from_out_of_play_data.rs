use board::space_identifiers::SpaceIdentifiers;

#[derive(Debug, Clone)]
pub struct DeployFromOutOfPlayData {
    forces_type: String,
    amount: u8,
    space_identifier: SpaceIdentifiers,
}

impl DeployFromOutOfPlayData {
    pub fn new(
        forces_type: String,
        amount: u8,
        space_identifier: SpaceIdentifiers,
    ) -> DeployFromOutOfPlayData {
        DeployFromOutOfPlayData {
            forces_type,
            amount,
            space_identifier,
        }
    }

    pub fn get_forces_type(&self) -> &String {
        &self.forces_type
    }

    pub fn get_amount(&self) -> u8 {
        self.amount
    }

    pub fn get_space_identifier(&self) -> SpaceIdentifiers {
        self.space_identifier
    }
}
