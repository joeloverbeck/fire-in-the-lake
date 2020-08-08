use board::map::Map;
use board::space_identifiers::SpaceIdentifiers;
use board::track::Track;
use commands::manipulate_aid::ManipulateAid;
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

pub fn execute_commands(
    card_number: u8,
    faction: Factions,
    commands: Vec<String>,
    map: &mut Map,
    track: &mut Track,
) -> Result<(), String> {
    // Depending on the card, faction, commands, etc. this function coordinates and delegates all the
    // possible deviations and instantation of executor commands so the map and track changes appropriately.

    if faction == Factions::VC && commands[0] == "event" {
        // Intends to execute the shaded event for a card and for the VC faction
        execute_shaded_event_for_vc(card_number, commands, map, track)?;

        Ok(())
    } else {
        todo!();
    }
}
