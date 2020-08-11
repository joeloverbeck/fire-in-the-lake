use factions::Factions;
use game_flow::factions_possibilities_handler::FactionsPossibilitiesHandler;

pub fn faction_should_be_movable_sanity_check(
    faction_to_move: Factions,
    factions_possibilities_handler: &FactionsPossibilitiesHandler,
) -> Result<(), String> {
    if !factions_possibilities_handler.is_faction_eligible(faction_to_move) {
        return Err(format!(
            "The faction to move {:?} wasn't eligible to begin with.",
            faction_to_move
        ));
    }

    Ok(())
}
