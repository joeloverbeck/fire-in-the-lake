
use decision_making::interpretation::transform_typed_space_into_space_identifier::transform_typed_space_into_space_identifier;
use decision_making::interpretation::operation_instructions::infiltrate_instructions::InfiltrateInstructions;
use board::track::Track;
use board::map::Map;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;
use board::space::Space;

pub fn register_infiltrate_instructions(split_typed_input_commands: Vec<&str>,
    interpreted_intentions: &mut InterpretedIntentions) -> Result<(), String> {
        // Wants to register an instruction for infiltration
        // Format: infiltration_instructions:[place/exchange/replace]:[space]:[vc_piece]

        if split_typed_input_commands[1] == "place"{
            interpreted_intentions.add_infiltrate_instructions(InfiltrateInstructions::new(split_typed_input_commands[1].to_string(), transform_typed_space_into_space_identifier(split_typed_input_commands[2]), None));
        }
        else if split_typed_input_commands[1] == "exchange"{
            interpreted_intentions.add_infiltrate_instructions(InfiltrateInstructions::new(split_typed_input_commands[1].to_string(), transform_typed_space_into_space_identifier(split_typed_input_commands[2]), None));
        }
        else if split_typed_input_commands[1] == "replace"{
            // Note: if the length of the split_typed_input_commands isn't as expected (should be an entry for the vc piece in question), this should crash
            if split_typed_input_commands.len() != 4 {
                panic!("Attempted to register a 'replace' type of infiltrate instruction, but the contents of the input commands weren't as expected: {:?}", split_typed_input_commands);
            }
            interpreted_intentions.add_infiltrate_instructions(InfiltrateInstructions::new(split_typed_input_commands[1].to_string(), transform_typed_space_into_space_identifier(split_typed_input_commands[2]), Some(split_typed_input_commands[3].to_string())));
        }
        else {
            return Err(format!("Was supposed to register infiltrate instructions, but the case for '{:?}' wasn't contemplated.", split_typed_input_commands[1]));
        }

        Ok(())
}