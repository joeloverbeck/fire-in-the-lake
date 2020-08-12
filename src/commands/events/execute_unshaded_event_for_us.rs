use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::atomic::deploy_force_type_from_out_of_play::deploy_force_type_from_out_of_play;
use commands::special_activities::execute_air_strike::execute_air_strike;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;

pub fn execute_unshaded_event_for_us(
    card_number: u8,
    interpreted_intentions: InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    match card_number {
        1 => {
            // Free air strike. Performed according to standard rules of that activity.
            // Can take six US pieces from out of play and place them in any city.

            // the vec of spaces for event should have the location for the air strike.
            execute_air_strike(
                interpreted_intentions.get_spaces_for_event()[0],
                map,
                track,
                available_forces,
            )?;

            // Additionally it can take 6 pieces from out of play and place them in any cities.
            // This will require accessing the specific instructions inside interpreted_intentions
            let deploy_from_out_of_play_data =
                interpreted_intentions.get_deploy_from_out_of_play_data();

            for entry in deploy_from_out_of_play_data.iter() {
                deploy_force_type_from_out_of_play(
                    entry.get_forces_type(),
                    entry.get_amount(),
                    entry.get_space_identifier(),
                    map,
                    available_forces,
                );
            }
        }
        _ => todo!(),
    }

    Ok(())
}
