use decision_making::choices::Choices;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;
use factions::Factions;

pub struct Decision {
    faction: Factions,
    choice: Choices,
    interpreted_intentions: InterpretedIntentions,
}

impl Decision {
    pub fn new(
        faction: Factions,
        new_choice: Choices,
        interpreted_intentions: InterpretedIntentions,
    ) -> Decision {
        Decision {
            faction,
            choice: new_choice,
            interpreted_intentions,
        }
    }

    pub fn get_faction(&self) -> Factions {
        self.faction
    }

    pub fn get_choice(&self) -> Choices {
        self.choice
    }

    pub fn get_interpreted_intentions(&self) -> InterpretedIntentions {
        self.interpreted_intentions.clone()
    }
}
