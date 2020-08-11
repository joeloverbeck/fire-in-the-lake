use board::current_rvn_leader::CurrentRvnLeader;
use board::map::Map;
use board::space::Space;
use board::track::Track;
use commands::manipulate_aid::manipulate_aid;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;

pub fn execute_special_activity_for_arvn(
    interpreted_intentions: InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
) -> Result<(), String> {
    if interpreted_intentions.does_it_want_to_govern() {
        for space_for_special_activity in interpreted_intentions.get_spaces_for_special_activity() {
            // Govern increases Aid. +3 * Space's Pop.
            let retrieved_space = map.get_space(space_for_special_activity).unwrap();

            manipulate_aid(track, (retrieved_space.get_population_value() * 3) as i8)?;
        }

        // Additionally, the current ARVN leader might have a bonus for Govern.
        if map.get_current_rvn_leader() == CurrentRvnLeader::Minh {
            // ARVN receives +5 Aid every time it trains
            manipulate_aid(track, 5)?;
        }
    } else {
        todo!()
    }

    Ok(())
}
