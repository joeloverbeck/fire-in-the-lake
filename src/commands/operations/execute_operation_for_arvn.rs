use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::operations::execute_pacify_for_arvn::execute_pacify_for_arvn;
use commands::operations::execute_train_for_arvn::execute_train_for_arvn;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;

pub fn execute_operation_for_arvn(
    interpreted_intentions: InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // In commands[1] we should have the name of the operation as well as where it is performed.
    if interpreted_intentions.does_it_want_to_train() {
        // This player chose to train.
        // For now we just handle one location, because I haven't come across another case.
        // The player should have specified the number of troops to put into the location.
        execute_train_for_arvn(
            interpreted_intentions.get_spaces_for_operation()[0],
            interpreted_intentions.get_digits_for_operation()[0],
            map,
            track,
            available_forces,
        )?;

        if interpreted_intentions.does_it_want_to_pacify() {
            // Wants to pacify.
            // It's implied what he wants to do, and he can only do it in the place
            // where he has trained.
            execute_pacify_for_arvn(
                interpreted_intentions.get_spaces_for_operation()[0],
                map,
                track,
            )?;
        }
    } else {
        todo!()
    }

    Ok(())
}
