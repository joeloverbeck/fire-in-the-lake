use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::events::execute_unshaded_event_for_us::execute_unshaded_event_for_us;
use commands::operations::sweep::sweep;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;

pub fn execute_commands_for_us(
    card_number: u8,
    interpreted_intentions: InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> std::result::Result<(), String> {
    if interpreted_intentions.does_it_want_to_activate_the_event() {
        // Intends to execute the unshaded event for a card and for the US faction
        execute_unshaded_event_for_us(
            card_number,
            interpreted_intentions,
            map,
            track,
            available_forces,
        )?;
    } else if interpreted_intentions.does_it_want_to_sweep() {
        if let Err(error) = sweep(interpreted_intentions.get_spaces_for_operation()[0], map) {
            panic!("There was an error while performing sweep: {:?}. The interpreted intentions of the US player were the following: {:?}", error, interpreted_intentions)
        }
    } else {
        todo!()
    }

    Ok(())
}
