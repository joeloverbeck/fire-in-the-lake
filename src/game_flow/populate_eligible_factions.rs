use factions::Factions;

pub fn populate_eligible_factions(faction_order: [Factions; 4], eligible: &mut [Factions; 4]) {
    eligible[0] = faction_order[0];
    eligible[1] = faction_order[1];
    eligible[2] = faction_order[2];
    eligible[3] = faction_order[3];
}
