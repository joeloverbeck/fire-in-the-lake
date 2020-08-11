use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::events::execute_shaded_event_for_vc::execute_shaded_event_for_vc;
use commands::events::execute_unshaded_event_for_arvn::execute_unshaded_event_for_arvn;
use commands::operations::execute_operation_for_arvn::execute_operation_for_arvn;
use commands::operations::execute_operation_for_nva::execute_operation_for_nva;
use commands::operations::execute_operation_for_vc::execute_operation_for_vc;
use commands::passing::execute_passed_for_arvn::execute_passed_for_nva;
use commands::special_activities::execute_special_activity_for_arvn::execute_special_activity_for_arvn;
use commands::special_activities::execute_special_activity_for_vc::execute_special_activity_for_vc;
use commands::sweep::sweep;
use decision_making::interpretation::interpreted_intentions::InterpretedIntentions;
use factions::Factions;

pub fn execute_commands(
    card_number: u8,
    faction: Factions,
    interpreted_intentions: InterpretedIntentions,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // Depending on the card, faction, commands, etc. this function coordinates and delegates all the
    // possible deviations and instantiations of executor commands so the map and track changes appropriately.
    match faction {
        Factions::VC => {
            if interpreted_intentions.does_it_want_to_activate_the_event() {
                // Intends to execute the shaded event for a card and for the VC faction
                execute_shaded_event_for_vc(card_number, interpreted_intentions, map, track)?;
            } else if interpreted_intentions.does_it_want_to_do_an_operation() {
                execute_operation_for_vc(
                    interpreted_intentions.clone(),
                    map,
                    track,
                    available_forces,
                )?;
                execute_special_activity_for_vc(interpreted_intentions, map, track)?;
            } else {
                todo!()
            }
            Ok(())
        }
        Factions::NVA => {
            if interpreted_intentions.does_it_want_to_pass() {
                // Has passed. Must receive some resources.
                execute_passed_for_nva(track)?;
            } else if interpreted_intentions.does_it_want_to_do_an_operation_only() {
                execute_operation_for_nva(
                    interpreted_intentions,
                    map,
                    track,
                    available_forces,
                    false,
                )?;
            } else {
                todo!()
            }

            Ok(())
        }
        Factions::ARVN => {
            if interpreted_intentions.does_it_want_to_do_an_operation() {
                // Has decided to do an operation.
                execute_operation_for_arvn(
                    interpreted_intentions.clone(),
                    map,
                    track,
                    available_forces,
                )?;
                execute_special_activity_for_arvn(interpreted_intentions, map, track)?;
            } else if interpreted_intentions.does_it_want_to_activate_the_event() {
                // Intends to execute the unshaded event for a card and for the ARVN faction
                execute_unshaded_event_for_arvn(
                    card_number,
                    interpreted_intentions,
                    map,
                    track,
                    available_forces,
                )?;
            } else {
                todo!()
            }

            Ok(())
        }
        Factions::US => {
            if interpreted_intentions.does_it_want_to_sweep() {
                if let Err(error) = sweep(interpreted_intentions.get_spaces_for_operation()[0], map)
                {
                    panic!("There was an error while performing sweep: {:?}. The interpreted intentions of the US player were the following: {:?}", error, interpreted_intentions)
                }
            }

            Ok(())
        }
        _ => todo!(),
    }
}
