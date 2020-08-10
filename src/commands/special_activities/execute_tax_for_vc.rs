use board::get_space_from_map::get_space_from_map;
use board::map::Map;
use board::space::Space;
use board::track::Track;
use decision_making::input_commands::InputCommands;

pub fn execute_tax_for_vc(
    tax_spaces: Vec<InputCommands>,
    map: &mut Map,
    track: &mut Track,
) -> Result<(), String> {
    // Must flip an underground vc guerrilla at each location in order to perform this operation.
    for tax_space_identifier in tax_spaces {
        let tax_space = get_space_from_map(tax_space_identifier, map)?;

        // Will add all the population values of the places, and then double them to add them to vc resources.
        let mut sum_population_values: u8 = 0;

        // Likely can only be done if there's at least an underground guerrilla there.
        if tax_space.get_number_of_underground_vc_guerrillas() > 0 {
            // Flip an underground vc guerrilla to active.
            tax_space.set_number_of_underground_vc_guerrillas(
                tax_space.get_number_of_underground_vc_guerrillas() - 1,
            );
            tax_space.set_number_of_active_vc_guerrillas(
                tax_space.get_number_of_active_vc_guerrillas() + 1,
            );

            sum_population_values += tax_space.get_population_value();

            // Must shift the population's support up a level.
            println!(
                "Support level of {:?} is {:?}",
                tax_space_identifier,
                tax_space.get_support_level()
            );
            tax_space.shift_support_level_up();
        }

        track.set_vc_resources(track.get_vc_resources() + (sum_population_values * 2));

        // Let's adjust the vc victory marker.
        track.adjust_vc_victory_marker(map);
    }

    Ok(())
}
