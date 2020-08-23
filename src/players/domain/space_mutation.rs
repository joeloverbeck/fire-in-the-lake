use game_definitions::space_identifiers::SpaceIdentifiers;
use players::domain::mutation_types::MutationTypes;
use players::domain::space_mutation_types::SpaceMutationTypes;
use players::domain::space_mutation_values::SpaceMutationValues;

#[derive(Debug)]
pub struct SpaceMutation {
    space_identifier: SpaceIdentifiers,
    space_mutation_types: SpaceMutationTypes,
    mutation_types: MutationTypes,
    space_mutation_values: Option<SpaceMutationValues>,
}

impl SpaceMutation {
    pub fn new(
        space_identifier: SpaceIdentifiers,
        space_mutation_types: SpaceMutationTypes,
        mutation_types: MutationTypes,
        space_mutation_values: Option<SpaceMutationValues>,
    ) -> SpaceMutation {
        SpaceMutation {
            space_identifier,
            space_mutation_types,
            mutation_types,
            space_mutation_values,
        }
    }

    pub fn get_space_identifier(&self) -> &SpaceIdentifiers {
        &self.space_identifier
    }

    pub fn get_space_mutation_types(&self) -> &SpaceMutationTypes {
        &self.space_mutation_types
    }

    pub fn get_mutation_types(&self) -> &MutationTypes {
        &self.mutation_types
    }

    pub fn get_space_mutation_values(&self) -> &SpaceMutationValues {
        &self.space_mutation_values.as_ref().unwrap()
    }
}
