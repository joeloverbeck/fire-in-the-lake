use board::space_identifiers::SpaceIdentifiers;

#[derive(Debug, Clone)]
pub struct MarchOrder {
    troop_type: String,
    amount: u8,
    from: SpaceIdentifiers,
    to: SpaceIdentifiers,
}

impl MarchOrder {
    pub fn new(
        troop_type: String,
        amount: u8,
        from: SpaceIdentifiers,
        to: SpaceIdentifiers,
    ) -> MarchOrder {
        MarchOrder {
            troop_type,
            amount,
            from,
            to,
        }
    }

    pub fn get_troop_type(&self) -> &str {
        &self.troop_type
    }

    pub fn get_amount(&self) -> u8 {
        self.amount
    }

    pub fn get_from(&self) -> SpaceIdentifiers {
        self.from
    }

    pub fn get_to(&self) -> SpaceIdentifiers {
        self.to
    }
}
