use board::available_forces::AvailableForces;
use board::current_rvn_leader::CurrentRvnLeader;
use board::map::Map;
use board::space::Space;
use board::space_identifiers::translate_space_name_to_identifier;
use board::space_identifiers::SpaceIdentifiers;
use board::track::Track;
use commands::deploy_arvn_troops_from_available::DeployArvnTroopsFromAvailable;
use commands::deploy_nva_guerrillas_from_available::DeployNvaGuerrillasFromAvailable;
use commands::deploy_us_irregulars_from_available::deploy_us_irregulars_from_available;
use commands::extract_multiword_command::extract_multiword_command;
use commands::improve_trail_nva::ImproveTrailNva;
use commands::manipulate_aid::ManipulateAid;
use commands::manipulate_arvn_resources::ManipulateArvnResources;
use commands::manipulate_nva_resources::ManipulateNvaResources;
use commands::set_space_to_active_support::set_space_to_active_support;
use commands::shift_support_of_space::ShiftSupportOfSpace;
use commands::sweep::Sweep;
use factions::Factions;

fn execute_shaded_event_for_vc(
    card_number: u8,
    _commands: Vec<String>,
    map: &mut Map,
    track: &mut Track,
) -> Result<(), String> {
    // Depending on the card (one of many, many, many), well execute one thing or another.
    match card_number {
        107 => {
            // Shift down support level in Saigon. Adjust Victory Markers. Lower aid.
            ShiftSupportOfSpace::new()
                .execute(map.get_space_mut(SpaceIdentifiers::Saigon).unwrap(), -1)?;
            ManipulateAid::new().execute(track, -12)?;
            track.adjust_us_victory_marker(&map);

            Ok(())
        }
        _ => Ok(()),
    }
}

fn execute_unshaded_event_for_arvn(
    card_number: u8,
    commands: Vec<String>,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    match card_number {
        68 => {
            // Place 3 irregulars or 3 rangers in a Province without NVA Control.
            // Set it to Active Support.
            deploy_us_irregulars_from_available(&commands[1], 3, map, track, available_forces)?;

            set_space_to_active_support(&commands[1], map, track)?;
        }
        _ => todo!(),
    }

    Ok(())
}

fn execute_train_for_arvn(
    location: &str,
    number_of_troops: u8,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // For training in a location, ARVN spends 3 of its resources,
    // and then gets troops (6 as far as I know, likely changeable)
    // in the location specified.
    ManipulateArvnResources::new().execute(track, -3)?;
    DeployArvnTroopsFromAvailable::new().execute(
        &String::from(location),
        number_of_troops,
        map,
        available_forces,
    )?;

    Ok(())
}

fn execute_pacify_for_arvn(location: &str, map: &mut Map, track: &mut Track) -> Result<(), String> {
    // It costs three resources.
    track.set_arvn_resources(track.get_arvn_resources() - 3);

    // Shift support upwards for the location.
    let space_identifier = translate_space_name_to_identifier(&String::from(location));
    let retrieved_space = map.get_space_mut(space_identifier).unwrap();

    ShiftSupportOfSpace::new().execute(retrieved_space, 1)?;

    // Trigger US' victory marker calculation.
    track.adjust_us_victory_marker(map);

    Ok(())
}

fn execute_rally_for_nva(
    locations: Vec<String>,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // Pays a resource cost of 1 per space.
    let resources_to_pay: i8 = locations.len() as i8;

    ManipulateNvaResources::new().execute(track, -resources_to_pay)?;

    for location in locations.iter() {
        DeployNvaGuerrillasFromAvailable::new().execute(location, map, track, available_forces)?;
    }

    Ok(())
}

fn execute_special_activity_for_arvn(
    commands: Vec<String>,
    map: &mut Map,
    track: &mut Track,
) -> Result<(), String> {
    if commands[0] == "govern" {
        let mut index = 1;

        while commands[index] != "stop" {
            // commands[index] should contain the name of the place where you want to Govern.
            // Govern increases Aid. +3 * Space's Pop.

            // retrieve the corresponding location
            let space_identifier = translate_space_name_to_identifier(&commands[index]);

            let retrieved_space = map.get_space(space_identifier).unwrap();

            ManipulateAid::new()
                .execute(track, (retrieved_space.get_population_value() * 3) as i8)?;

            index += 1;
        }

        // Additionally, the current ARVN leader might have a bonus for Govern.
        if map.get_current_rvn_leader() == CurrentRvnLeader::Minh {
            // ARVN receives +5 Aid every time it trains
            ManipulateAid::new().execute(track, 5)?;
        }
    } else {
        todo!()
    }

    Ok(())
}

fn execute_operation_for_nva(
    commands: &[String],
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
    _special_activity: bool,
) -> Result<(), String> {
    // In commands[1] we should have the name of the operation.
    // From then onwards, until a 'stop', should be the locations where it is performed
    if commands[1] == "rally" {
        // Let's extract all the locations where it intends to rally.
        let mut command_locations: Vec<String> = Vec::new();

        for item in commands.iter().skip(2) {
            if item == "stop" {
                break;
            }

            command_locations.push(item.to_string());
        }

        execute_rally_for_nva(command_locations, map, track, available_forces)?;

        // As part of choosing rally, they get the opportunity to improve the trail
        // one level.
        let index_of_last_command = commands
            .iter()
            .position(|command| command == "stop")
            .unwrap();

        if commands[index_of_last_command + 1] == "yes" {
            // wants to improve the trail.
            ImproveTrailNva::new().execute(track)?;
        }
    } else {
        todo!()
    }

    Ok(())
}

fn execute_operation_for_arvn(
    commands: &[String],
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // In commands[1] we should have the name of the operation as well as where it is performed.
    let operation_command = extract_multiword_command(&commands[1]);

    if operation_command[0] == "train" {
        // This player chose to train.
        // For now we just handle one location, because I haven't come across another case.
        // The player should have specified the number of troops to put into the location.
        let number_of_troops: u8 = commands[2].parse::<u8>().unwrap();

        execute_train_for_arvn(
            &operation_command[1],
            number_of_troops,
            map,
            track,
            available_forces,
        )?;

        // Additionally, the player might have chosen to Pacify.
        if commands[3] == "no" {
            // Doesn't want to pacify.
            return Ok(());
        }

        // Wants to pacify.
        // It's implied what he wants to do, and he can only do it in the place
        // where he has trained.
        execute_pacify_for_arvn(&operation_command[1], map, track)?;
    } else {
        todo!()
    }

    Ok(())
}

fn execute_passed_for_nva(track: &mut Track) -> Result<(), String> {
    // NVA passing increases their resources +1.
    ManipulateNvaResources::new().execute(track, 1)?;

    Ok(())
}

pub fn execute_commands(
    card_number: u8,
    faction: Factions,
    commands: Vec<String>,
    map: &mut Map,
    track: &mut Track,
    available_forces: &mut AvailableForces,
) -> Result<(), String> {
    // Depending on the card, faction, commands, etc. this function coordinates and delegates all the
    // possible deviations and instantiations of executor commands so the map and track changes appropriately.
    match faction {
        Factions::VC => {
            if commands[0] == "event" {
                // Intends to execute the shaded event for a card and for the VC faction
                execute_shaded_event_for_vc(card_number, commands, map, track)?;
            } else {
                todo!()
            }
            Ok(())
        }
        Factions::NVA => {
            if commands[0] == "pass" {
                // Has passed. Must receive some resources.
                execute_passed_for_nva(track)?;
            } else if commands[0] == "operation only" || commands[0] == "op only" {
                execute_operation_for_nva(&commands, map, track, available_forces, false)?;
            } else {
                todo!()
            }

            Ok(())
        }
        Factions::ARVN => {
            if commands[0] == "operation" {
                // Has decided to do an operation.
                execute_operation_for_arvn(&commands, map, track, available_forces)?;

                // Additionally, he might have chosen to perform a Special Activity
                if commands[4] == "no" {
                    // He doesn't want to.
                    return Ok(());
                }

                // in commands[5] should be the name of the special activity
                let mut special_activity_commands: Vec<String> = Vec::new();

                for n in commands.iter().skip(5) {
                    special_activity_commands.push(n.to_owned());
                }

                execute_special_activity_for_arvn(special_activity_commands, map, track)?;
            } else if commands[0] == "event" {
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
            if commands[0] == "sweep" {
                Sweep::new().execute(&commands[1], map)?;
            }

            Ok(())
        }
        _ => todo!(),
    }
}
