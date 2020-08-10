use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use collections::create_vec_of_vec_from_index::create_vec_of_vec_from_index;
use collections::get_index_of_next_batch_of_commands_after_stop::get_index_of_next_batch_of_commands_after_stop;
use commands::events::execute_shaded_event_for_vc::execute_shaded_event_for_vc;
use commands::events::execute_unshaded_event_for_arvn::execute_unshaded_event_for_arvn;
use commands::operations::execute_operation_for_arvn::execute_operation_for_arvn;
use commands::operations::execute_operation_for_nva::execute_operation_for_nva;
use commands::operations::execute_operation_for_vc::execute_operation_for_vc;
use commands::passing::execute_passed_for_arvn::execute_passed_for_nva;
use commands::special_activities::execute_special_activity_for_arvn::execute_special_activity_for_arvn;
use commands::special_activities::execute_special_activity_for_vc::execute_special_activity_for_vc;
use commands::sweep::sweep;
use decision_making::input_commands::InputCommands;
use factions::Factions;

pub fn execute_commands(
    card_number: u8,
    faction: Factions,
    commands: Vec<InputCommands>,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // Depending on the card, faction, commands, etc. this function coordinates and delegates all the
    // possible deviations and instantiations of executor commands so the map and track changes appropriately.
    match faction {
        Factions::VC => {
            if commands[0] == InputCommands::Event {
                // Intends to execute the shaded event for a card and for the VC faction
                execute_shaded_event_for_vc(card_number, commands, map, track)?;
            } else if commands[0] == InputCommands::Operation {
                // Has decided to do an operation.
                let operation_commands = create_vec_of_vec_from_index(&commands, 1)?;

                execute_operation_for_vc(operation_commands, map, track, available_forces)?;

                let special_activity_commands = create_vec_of_vec_from_index(
                    &commands,
                    get_index_of_next_batch_of_commands_after_stop(&commands),
                )?;

                if special_activity_commands[0] == InputCommands::Yes {
                    execute_special_activity_for_vc(special_activity_commands, map, track)?;
                }
            } else {
                todo!()
            }
            Ok(())
        }
        Factions::NVA => {
            if commands[0] == InputCommands::Pass {
                // Has passed. Must receive some resources.
                execute_passed_for_nva(track)?;
            } else if commands[0] == InputCommands::OperationOnly {
                execute_operation_for_nva(commands, map, track, available_forces, false)?;
            } else {
                todo!()
            }

            Ok(())
        }
        Factions::ARVN => {
            if commands[0] == InputCommands::Operation {
                // Has decided to do an operation.
                execute_operation_for_arvn(commands.clone(), map, track, available_forces)?;

                // Additionally, he might have chosen to perform a Special Activity
                if commands[5] == InputCommands::No {
                    // He doesn't want to.
                    return Ok(());
                }

                // in commands[6] should be the name of the special activity
                let mut special_activity_commands: Vec<InputCommands> = Vec::new();

                for n in commands.iter().skip(6) {
                    special_activity_commands.push(n.to_owned());
                }

                execute_special_activity_for_arvn(special_activity_commands, map, track)?;
            } else if commands[0] == InputCommands::Event {
                // Intends to execute the unshaded event for a card and for the ARVN faction
                execute_unshaded_event_for_arvn(
                    card_number,
                    commands,
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
            if commands[0] == InputCommands::Sweep {
                sweep(commands[1], map)?;
            }

            Ok(())
        }
        _ => todo!(),
    }
}
