use board::available_forces::AvailableForces;
use board::map::Map;
use board::space::Space;
use board::space_identifiers::SpaceIdentifiers;

pub fn deploy_force_type_from_out_of_play(
    force_type: &str,
    amount: u8,
    space_identifier: SpaceIdentifiers,
    map: &mut Map,
    available_forces: &mut AvailableForces,
) {
    if force_type == "us_troops" {
        // Sanity check
        if available_forces.get_out_of_play_us_troops() < amount {
            panic!("Attempted to deploy {:?} us troops from the out of play box, but in there there were {:?}", amount, available_forces.get_out_of_play_us_troops());
        }

        available_forces
            .set_out_of_play_us_troops(available_forces.get_out_of_play_us_troops() - amount);

        let retrieved_space = map.get_space_mut(space_identifier).unwrap();

        retrieved_space.set_number_of_us_troops(retrieved_space.get_number_of_us_troops() + amount);
    } else if force_type == "us_base" {
        // Sanity check
        if available_forces.get_out_of_play_us_bases() < amount {
            panic!("Attempted to deploy {:?} us bases from the out of play box, but in there there were {:?}", amount, available_forces.get_out_of_play_us_bases());
        }

        available_forces
            .set_out_of_play_us_bases(available_forces.get_out_of_play_us_bases() - amount);

        let retrieved_space = map.get_space_mut(space_identifier).unwrap();

        retrieved_space.set_number_of_us_bases(retrieved_space.get_number_of_us_bases() + amount);
    } else {
        todo!()
    }
}
