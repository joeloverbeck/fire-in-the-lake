use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::operations::execute_rally_for_nva::execute_rally_for_nva;
use commands::operations::improve_trail_nva::improve_trail_nva;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;

pub fn execute_operation_for_nva(
    interpreted_intentions: InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
    _special_activity: bool,
) -> Result<(), String> {
    // In commands[1] we should have the name of the operation.
    // From then onwards, until a 'stop', should be the locations where it is performed
    if interpreted_intentions.does_it_want_to_rally() {
        execute_rally_for_nva(
            interpreted_intentions.get_spaces_for_operation(),
            map,
            track,
            available_forces,
        )?;

        // As part of choosing rally, they get the opportunity to improve the trail
        // one level.
        if interpreted_intentions.does_it_want_to_improve_the_trail() {
            // wants to improve the trail.
            improve_trail_nva(track)?;
        }
    } else {
        todo!()
    }

    Ok(())
}
