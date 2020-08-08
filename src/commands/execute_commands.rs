use board::map::Map;
use board::space_identifiers::SpaceIdentifiers;
use board::track::Track;
use commands::manipulate_aid::ManipulateAid;
use commands::manipulate_nva_resources::ManipulateNvaResources;
use commands::shift_support_of_space::ShiftSupportOfSpace;
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
) -> Result<(), String> {
    // Depending on the card, faction, commands, etc. this function coordinates and delegates all the
    // possible deviations and instantation of executor commands so the map and track changes appropriately.
    match faction {
        Factions::VC => {
            if commands[0] == "event" {
                // Intends to execute the shaded event for a card and for the VC faction
                execute_shaded_event_for_vc(card_number, commands, map, track)?;
            }
            Ok(())
        }
        Factions::NVA => {
            if commands[0] == "pass" {
                // Has passed. Must receive some resources.
                execute_passed_for_nva(track)?;
            }

            Ok(())
        }
        _ => Ok(()),
    }
}
