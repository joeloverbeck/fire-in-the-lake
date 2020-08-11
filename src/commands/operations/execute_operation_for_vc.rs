use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::operations::execute_rally_for_vc::execute_rally_for_vc;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;

pub fn execute_operation_for_vc(
    interpreted_intentions: InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    if interpreted_intentions.does_it_want_to_rally() {
        execute_rally_for_vc(
            interpreted_intentions.get_spaces_for_operation(),
            map,
            track,
            available_forces,
        )?;
    } else {
        todo!()
    }

    Ok(())
}
