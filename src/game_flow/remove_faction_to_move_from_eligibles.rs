use factions::Factions;

pub fn remove_faction_to_move_from_eligibles(
    faction_to_move: Factions,
    eligible: &mut [Factions; 4],
) -> Result<(), String> {
    // First we gotta make sure it's there. Nobody should call this function if the faction to remove isn't
    // in the elegibles to begin with.
    if !eligible
        .iter()
        .any(|eligible_faction| eligible_faction == &faction_to_move)
    {
        return Err(format!("Was ordered to remove the faction {:?} from the list of elegibles, but it wasn't there in the first place. The list is: {:?}", faction_to_move, eligible));
    }

    for item in eligible.iter_mut() {
        match *item {
            _ if item == &faction_to_move => *item = Factions::None,
            _ => (),
        }
    }

    Ok(())
}
