use board::available_forces::AvailableForces;
use board::map::Map;
use board::space_identifiers::SpaceIdentifiers;
use board::track::Track;
use commands::atomic::move_troops_from_space_to_space::move_troops_from_space_to_space;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;

pub fn execute_march_for_nva(
    interpreted_intentions: &InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
    _available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // Sanity check
    if interpreted_intentions.get_march_orders().is_empty() {
        panic!("Had to execute a march for NVA, but there were no march orders!");
    }

    // Need to know how many destinations there are.
    let mut destinations: Vec<SpaceIdentifiers> = Vec::new();

    for march_order in interpreted_intentions.get_march_orders().iter() {
        if destinations
            .iter()
            .find(|destination| *destination == &march_order.get_to())
            .is_none()
        {
            destinations.push(march_order.get_to());
        }
    }

    if track.get_nva_resources() < destinations.len() as u8 {
        panic!("Was processing a march for NVA, but there were more destinations than nva resources! {:?} to {:?}. The code that handles the player input should have taken care of it.", destinations.len(), track.get_nva_resources());
    }

    // Will decrease NVA resources according to the amount of destinations.
    track.set_nva_resources(track.get_nva_resources() - destinations.len() as u8);

    for march_order in interpreted_intentions.get_march_orders().iter() {
        // Now just perform each move.
        move_troops_from_space_to_space(
            march_order.get_troop_type(),
            march_order.get_amount(),
            march_order.get_from(),
            march_order.get_to(),
            map,
        )?;
    }

    Ok(())
}
