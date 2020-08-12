use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::events::execute_shaded_event_for_vc::execute_shaded_event_for_vc;
use commands::operations::execute_operation_for_vc::execute_operation_for_vc;
use commands::special_activities::execute_special_activity_for_vc::execute_special_activity_for_vc;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;

pub fn execute_commands_for_vc(
    card_number: u8,
    interpreted_intentions: InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> std::result::Result<(), String> {
    if interpreted_intentions.does_it_want_to_activate_the_event() {
        // Intends to execute the shaded event for a card and for the VC faction
        execute_shaded_event_for_vc(card_number, interpreted_intentions, map, track)?;
    } else if interpreted_intentions.does_it_want_to_do_an_operation() {
        execute_operation_for_vc(interpreted_intentions.clone(), map, track, available_forces)?;
        execute_special_activity_for_vc(interpreted_intentions, map, track)?;
    } else {
        todo!()
    }
    Ok(())
}
