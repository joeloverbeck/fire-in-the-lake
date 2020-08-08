use decision_making::choices::Choices;
use factions::Factions;

pub struct Decision {
    faction: Factions,
    choice: Choices,
    commands: Vec<String>,
}

impl Decision {
    pub fn new(faction: Factions, new_choice: Choices, commands: Vec<String>) -> Decision {
        Decision {
            faction,
            choice: new_choice,
            commands,
        }
    }

    pub fn get_faction(&self) -> Factions {
        self.faction
    }

    pub fn get_choice(&self) -> Choices {
        self.choice
    }

    pub fn get_commands(&self) -> Vec<String> {
        self.commands.to_owned()
    }
}
