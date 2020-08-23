use players::domain::mutations::Mutations;

#[derive(Debug)]
pub struct Decision {
    mutations: Mutations,
}

impl Decision {
    pub fn new(mutations: Mutations) -> Decision {
        Decision { mutations }
    }

    pub fn get_mutations(&self) -> Result<&Mutations, String> {
        Ok(&self.mutations)
    }
}
