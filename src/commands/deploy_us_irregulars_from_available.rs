use board::available_forces::AvailableForces;
use board::map::Map;
use board::space::Space;
use board::space_identifiers::SpaceIdentifiers;
use board::track::Track;
use math::amount_that_was_removed_from_number::amount_that_was_removed_from_number;

pub fn deploy_us_irregulars_from_available(
    space_identifier: SpaceIdentifiers,
    amount_of_us_irregulars: u8,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    let amount_of_us_irregulars_removed: u8 = amount_that_was_removed_from_number(
        available_forces.remove_amount_of_us_irregulars(amount_of_us_irregulars),
    );

    let retrieved_space = map.get_space_mut(space_identifier);

    if let Ok(space) = retrieved_space {
        space.set_number_of_underground_special_forces_irregulars(
            space.get_number_of_underground_special_forces_irregulars()
                + amount_of_us_irregulars_removed,
        );

        track.adjust_control_plus_patronage(map);
    }

    Ok(())
}
