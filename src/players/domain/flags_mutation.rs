use game_definitions::flags::Flags;
use players::domain::mutation_types::MutationTypes;

#[derive(Debug)]
pub struct FlagsMutation {
    flag: Flags,
    mutation_type: MutationTypes,
}

impl FlagsMutation {
    pub fn new(flag: Flags, mutation_type: MutationTypes) -> FlagsMutation {
        FlagsMutation {
            flag,
            mutation_type,
        }
    }

    pub fn get_flag(&self) -> &Flags {
        &self.flag
    }

    pub fn get_mutation_type(&self) -> &MutationTypes {
        &self.mutation_type
    }
}
