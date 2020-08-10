use board::map::Map;
use board::track::Track;
use collections::extract_target_spaces_from_commands::extract_target_spaces_from_commands;
use commands::special_activities::execute_tax_for_vc::execute_tax_for_vc;
use decision_making::input_commands::InputCommands;

pub fn execute_special_activity_for_vc(
    commands: Vec<InputCommands>,
    map: &mut Map,
    track: &mut Track,
) -> Result<(), String> {
    if commands[1] == InputCommands::Tax {
        // Will tax in a certain number of locations, unknown beforehand,
        // but it should stop at stop.
        let tax_spaces = extract_target_spaces_from_commands(&commands, 2);

        execute_tax_for_vc(tax_spaces, map, track)?;
    } else {
        todo!()
    }

    Ok(())
}
