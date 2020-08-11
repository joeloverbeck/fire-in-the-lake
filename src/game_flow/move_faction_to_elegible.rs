use factions::Factions;

pub fn move_faction_to_eligible(faction_to_move: Factions, eligible: &mut [Factions; 4]) {
    for item in eligible.iter_mut() {
        match *item {
            _ if item == &Factions::None => {
                *item = faction_to_move;
                break;
            }
            _ => (),
        }
    }
}
