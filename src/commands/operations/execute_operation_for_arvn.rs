use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::operations::execute_pacify_for_arvn::execute_pacify_for_arvn;
use commands::operations::execute_train_for_arvn::execute_train_for_arvn;
use decision_making::input_commands::InputCommands;
use decision_making::translate_input_command_to_digit::translate_input_command_to_digit;

pub fn execute_operation_for_arvn(
    commands: Vec<InputCommands>,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // In commands[1] we should have the name of the operation as well as where it is performed.
    if commands[1] == InputCommands::Train {
        // This player chose to train.
        // For now we just handle one location, because I haven't come across another case.
        // The player should have specified the number of troops to put into the location.
        let number_of_troops: u8 = translate_input_command_to_digit(commands[3]);

        execute_train_for_arvn(commands[2], number_of_troops, map, track, available_forces)?;

        // Additionally, the player might have chosen to Pacify.
        if commands[3] == InputCommands::No {
            // Doesn't want to pacify.
            return Ok(());
        }

        // Wants to pacify.
        // It's implied what he wants to do, and he can only do it in the place
        // where he has trained.
        execute_pacify_for_arvn(commands[2], map, track)?;
    } else {
        todo!()
    }

    Ok(())
}
