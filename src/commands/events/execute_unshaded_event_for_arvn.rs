use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::deploy_us_irregulars_from_available::deploy_us_irregulars_from_available;
use commands::set_space_to_active_support::set_space_to_active_support;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;

pub fn execute_unshaded_event_for_arvn(
    card_number: u8,
    interpreted_intentions: InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    match card_number {
        68 => {
            // Place 3 irregulars or 3 rangers in a Province without NVA Control.
            // Set it to Active Support.
            deploy_us_irregulars_from_available(
                interpreted_intentions.get_spaces_for_event()[0],
                3,
                map,
                track,
                available_forces,
            )?;
            set_space_to_active_support(
                interpreted_intentions.get_spaces_for_event()[0],
                map,
                track,
            )?;
        }
        _ => todo!(),
    }

    Ok(())
}
