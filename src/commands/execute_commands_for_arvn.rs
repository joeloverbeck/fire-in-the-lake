use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::events::execute_unshaded_event_for_arvn::execute_unshaded_event_for_arvn;
use commands::operations::execute_operation_for_arvn::execute_operation_for_arvn;
use commands::special_activities::execute_special_activity_for_arvn::execute_special_activity_for_arvn;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;

pub fn execute_commands_for_arvn(
    card_number: u8,
    interpreted_intentions: InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> std::result::Result<(), String> {
    if interpreted_intentions.does_it_want_to_do_an_operation() {
        // Has decided to do an operation.
        execute_operation_for_arvn(interpreted_intentions.clone(), map, track, available_forces)?;
        execute_special_activity_for_arvn(interpreted_intentions, map, track)?;
    } else if interpreted_intentions.does_it_want_to_activate_the_event() {
        // Intends to execute the unshaded event for a card and for the ARVN faction
        execute_unshaded_event_for_arvn(
            card_number,
            interpreted_intentions,
            map,
            track,
            available_forces,
        )?;
    } else {
        todo!()
    }

    Ok(())
}
