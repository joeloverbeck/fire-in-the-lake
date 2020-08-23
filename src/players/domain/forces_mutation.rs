use game_definitions::forces::Forces;
use game_definitions::space_identifiers::SpaceIdentifiers;
use players::domain::mutation_types::MutationTypes;

#[derive(Debug, Clone, Copy)]
pub struct ForcesMutation {
    forces: Forces,
    mutation_type: MutationTypes,
    number: u8,
    from: Option<SpaceIdentifiers>,
    to: Option<SpaceIdentifiers>,
}

impl ForcesMutation {
    pub fn new(
        forces: Forces,
        mutation_type: MutationTypes,
        number: u8,
        from: Option<SpaceIdentifiers>,
        to: Option<SpaceIdentifiers>,
    ) -> ForcesMutation {
        ForcesMutation {
            forces,
            mutation_type,
            number,
            from,
            to,
        }
    }

    pub fn get_forces(&self) -> &Forces {
        &self.forces
    }

    pub fn get_mutation_type(&self) -> &MutationTypes {
        &self.mutation_type
    }

    pub fn get_number(&self) -> u8 {
        self.number
    }

    pub fn get_from(&self) -> &Option<SpaceIdentifiers> {
        &self.from
    }

    pub fn get_to(&self) -> &Option<SpaceIdentifiers> {
        &self.to
    }
}
