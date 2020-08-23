use players::domain::decision_information::DecisionInformation;
use players::domain::mutations::Mutations;

#[derive(Debug)]
pub struct Decision {
    mutations: Mutations,
    main_action: Option<DecisionInformation>,
    secondary_action: Option<DecisionInformation>,
}

impl Decision {
    pub fn new(
        mutations: Mutations,
        main_action: Option<DecisionInformation>,
        secondary_action: Option<DecisionInformation>,
    ) -> Decision {
        Decision {
            mutations,
            main_action,
            secondary_action,
        }
    }

    pub fn get_mutations(&self) -> Result<&Mutations, String> {
        Ok(&self.mutations)
    }

    pub fn has_main_action(&self) -> Result<bool, String> {
        Ok(self.main_action.is_some())
    }

    pub fn get_main_action(&self) -> Result<&DecisionInformation, String> {
        Ok(self.main_action.as_ref().unwrap())
    }

    pub fn has_secondary_action(&self) -> Result<bool, String> {
        Ok(self.secondary_action.is_some())
    }

    pub fn get_secondary_action(&self) -> Result<&DecisionInformation, String> {
        Ok(self.secondary_action.as_ref().unwrap())
    }
}
