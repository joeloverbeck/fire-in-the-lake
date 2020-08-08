use commands::command::Command;
use commands::command::Commands;
use decision_making::choices::Choices;
use std::collections::VecDeque;

pub struct Decision<'a> {
    choice: Choices,
    commands: VecDeque<Commands<'a>>,
}

impl<'a> Decision<'a> {
    pub fn new(
        new_choice: Choices,
        commands: std::collections::VecDeque<Commands<'a>>,
    ) -> Decision<'a> {
        Decision {
            choice: new_choice,
            commands,
        }
    }

    pub fn get_choice(&self) -> Choices {
        self.choice
    }

    pub fn execute_commands(&mut self) {
        // Should go through the queue from the first to the last one and execute each.
        self.commands
            .iter_mut()
            .for_each(|command| command.execute().unwrap());
    }
}
