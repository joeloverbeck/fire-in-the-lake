use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;

pub fn calculate_number_of_a_factions_special_rangers_in_space(
    faction: Factions,
    space: &Spaces,
) -> Result<u8, String> {
    match faction {
        Factions::NVA => Ok(space.get_forces(Forces::UndergroundNvaGuerrilla).unwrap()
            + space.get_forces(Forces::ActiveNvaGuerrilla).unwrap()),
        _ => panic!(
            "Calculate number of a factions special rangers not implemented for faction {:?}",
            faction
        ),
    }
}
