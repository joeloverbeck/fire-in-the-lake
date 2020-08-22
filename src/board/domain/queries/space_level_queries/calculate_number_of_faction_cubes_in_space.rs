use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;

pub fn calculate_number_of_faction_cubes_in_space(
    faction: Factions,
    occupable_space: &Spaces,
) -> Result<u8, String> {
    match faction {
        Factions::ARVN => Ok(occupable_space.get_forces(Forces::ArvnPolice)?
            + occupable_space.get_forces(Forces::ArvnTroop)?),
        Factions::US => Ok(occupable_space.get_forces(Forces::UsTroop)?),
        _ => panic!("Not implemented for faction {:?}", faction),
    }
}
