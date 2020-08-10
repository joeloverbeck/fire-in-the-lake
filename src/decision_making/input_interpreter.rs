use decision_making::input_commands::InputCommands;
use std::collections::HashMap;

pub struct InputInterpreter {
    interpretations: HashMap<String, InputCommands>,
}

impl Default for InputInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl InputInterpreter {
    pub fn new() -> InputInterpreter {
        let mut input_interpreter = InputInterpreter {
            interpretations: HashMap::new(),
        };

        input_interpreter.populate_interpretations();

        input_interpreter
    }

    pub fn populate_interpretations(&mut self) {
        // Insert all the possible correlations of string to InputCommands
        self.interpretations
            .insert("event".to_string(), InputCommands::Event);
        self.interpretations
            .insert("govern".to_string(), InputCommands::Govern);
        self.interpretations
            .insert("yes".to_string(), InputCommands::Yes);
        self.interpretations
            .insert("no".to_string(), InputCommands::No);
        self.interpretations
            .insert("6".to_string(), InputCommands::Six);
        self.interpretations
            .insert("operation".to_string(), InputCommands::Operation);
        self.interpretations
            .insert("operation only".to_string(), InputCommands::OperationOnly);
        self.interpretations
            .insert("op only".to_string(), InputCommands::OperationOnly);
        self.interpretations
            .insert("pass".to_string(), InputCommands::Pass);
        self.interpretations
            .insert("rally".to_string(), InputCommands::Rally);
        self.interpretations
            .insert("stop".to_string(), InputCommands::Stop);
        self.interpretations
            .insert("sweep".to_string(), InputCommands::Sweep);
        self.interpretations
            .insert("train".to_string(), InputCommands::Train);
        self.interpretations
            .insert("saigon".to_string(), InputCommands::Saigon);
        self.interpretations
            .insert("an loc".to_string(), InputCommands::AnLoc);
        self.interpretations
            .insert("can tho".to_string(), InputCommands::CanTho);
        self.interpretations
            .insert("north vietnam".to_string(), InputCommands::NorthVietnam);
        self.interpretations.insert(
            "the parrot's beak".to_string(),
            InputCommands::TheParrotsBeak,
        );
        self.interpretations
            .insert("kien phong".to_string(), InputCommands::KienPhong);
        self.interpretations
            .insert("kien giang".to_string(), InputCommands::KienGiang);
        self.interpretations
            .insert("quang tri".to_string(), InputCommands::QuangTri);
        self.interpretations
            .insert("binh dinh".to_string(), InputCommands::BinhDinh);
    }

    pub fn interpret(
        &self,
        typed_input_commands: Vec<String>,
    ) -> Result<Vec<InputCommands>, String> {
        let mut recognized_commands: Vec<InputCommands> = Vec::new();

        for input in typed_input_commands {
            let possible_interpretation = self.interpretations.get(&input);

            match possible_interpretation {
                Some(interpretation) => recognized_commands.push(*interpretation),
                None => {
                    return Err(format!(
                        "Failed to find an interpretation for '{:?}'",
                        input
                    ))
                }
            }
        }

        Ok(recognized_commands)
    }
}
