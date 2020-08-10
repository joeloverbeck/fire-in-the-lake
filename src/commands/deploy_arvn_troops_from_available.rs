use board::available_forces::AvailableForces;
use board::map::Map;
use board::space::Space;
use board::space_identifiers::translate_space_name_to_identifier;
use decision_making::input_commands::InputCommands;
use math::amount_that_was_removed_from_number::amount_that_was_removed_from_number;

pub fn deploy_arvn_troops_from_available(
    location: InputCommands,
    number_of_troops: u8,
    map: &mut Map,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    let space_identifier = translate_space_name_to_identifier(location);

    let amount_of_arvn_available_removed: u8 = amount_that_was_removed_from_number(
        available_forces.remove_amount_of_arvn_troops(number_of_troops),
    );

    let retrieved_space = map.get_space_mut(space_identifier);

    if let Ok(space) = retrieved_space {
        space.set_number_of_arvn_troops(
            space.get_number_of_arvn_troops() + amount_of_arvn_available_removed,
        )?;
    }

    Ok(())
}