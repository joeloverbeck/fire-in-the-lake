use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::deploy_us_irregulars_from_available::deploy_us_irregulars_from_available;
use commands::set_space_to_active_support::set_space_to_active_support;
use decision_making::input_commands::InputCommands;

pub fn execute_unshaded_event_for_arvn(
    card_number: u8,
    commands: Vec<InputCommands>,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    match card_number {
        68 => {
            // Place 3 irregulars or 3 rangers in a Province without NVA Control.
            // Set it to Active Support.
            deploy_us_irregulars_from_available(commands[1], 3, map, track, available_forces)?;

            set_space_to_active_support(commands[1], map, track)?;
        }
        _ => todo!(),
    }

    Ok(())
}
