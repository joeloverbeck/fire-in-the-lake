use board::map::Map;
use board::track::Track;
use commands::special_activities::execute_tax_for_vc::execute_tax_for_vc;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;

pub fn execute_special_activity_for_vc(
    interpreted_intentions: InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
) -> Result<(), String> {
    if interpreted_intentions.does_it_want_to_tax() {
        // Will tax in a certain number of locations, unknown beforehand,
        // but it should stop at stop.
        execute_tax_for_vc(
            interpreted_intentions.get_spaces_for_special_activity(),
            map,
            track,
        )?;
    } else {
        todo!()
    }

    Ok(())
}
