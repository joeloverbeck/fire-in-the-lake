use board::available_forces::AvailableForces;
use board::map::Map;
use board::track::Track;
use commands::execute_commands_for_arvn::execute_commands_for_arvn;
use commands::execute_commands_for_nva::execute_commands_for_nva;
use commands::execute_commands_for_us::execute_commands_for_us;
use commands::execute_commands_for_vc::execute_commands_for_vc;
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
        Factions::VC => execute_commands_for_vc(
            card_number,
            interpreted_intentions,
            map,
            track,
            available_forces,
        ),
        Factions::NVA => execute_commands_for_nva(
            card_number,
            interpreted_intentions,
            map,
            track,
            available_forces,
        ),
        Factions::ARVN => execute_commands_for_arvn(
            card_number,
            interpreted_intentions,
            map,
            track,
            available_forces,
        ),
        Factions::US => execute_commands_for_us(
            card_number,
            interpreted_intentions,
            map,
            track,
            available_forces,
        ),
        _ => todo!(),
    }
}
