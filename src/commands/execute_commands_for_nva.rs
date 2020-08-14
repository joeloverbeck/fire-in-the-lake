use commands::special_activities::execute_special_activity_for_nva::execute_special_activity_for_nva;
use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::operations::execute_operation_for_nva::execute_operation_for_nva;
use commands::passing::execute_passed_for_arvn::execute_passed_for_nva;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;

pub fn execute_commands_for_nva(
    _card_number: u8,
    interpreted_intentions: InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    if interpreted_intentions.does_it_want_to_pass() {
        // Has passed. Must receive some resources.
        execute_passed_for_nva(track)?;
    } else if interpreted_intentions.does_it_want_to_do_an_operation() {
        execute_operation_for_nva(&interpreted_intentions, map, track, available_forces)?;
        execute_special_activity_for_nva(&interpreted_intentions, map, track, available_forces)?;
    } else if interpreted_intentions.does_it_want_to_do_an_operation_only() {
        execute_operation_for_nva(&interpreted_intentions, map, track, available_forces)?;
    } else {
        todo!()
    }

    Ok(())
}
