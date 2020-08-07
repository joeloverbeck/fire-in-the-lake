use decision_making::choices::Choices;

pub struct Decision {
    choice: Choices,
}

impl Decision {
    pub fn new(new_choice: Choices) -> Decision {
        Decision { choice: new_choice }
    }

    pub fn get_choice(&self) -> Choices {
        self.choice
    }
}
