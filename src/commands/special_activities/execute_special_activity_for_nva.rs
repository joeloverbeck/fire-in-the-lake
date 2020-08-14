use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::special_activities::execute_infiltrate_for_nva::execute_infiltrate_for_nva;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;

pub fn execute_special_activity_for_nva(
    interpreted_intentions: &InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    if interpreted_intentions.does_it_want_to_infiltrate() {
        execute_infiltrate_for_nva(
            interpreted_intentions.get_infiltrate_instructions(),
            map,
            track,
            available_forces,
        )?;
    }

    Ok(())
}
