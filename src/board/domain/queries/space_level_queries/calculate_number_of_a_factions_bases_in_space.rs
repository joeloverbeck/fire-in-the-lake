use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;

pub fn calculate_number_of_a_factions_bases_in_space(
    faction: Factions,
    space: &Spaces,
) -> Result<u8, String> {
    let number_of_a_factions_bases_in_space = {
        match faction {
            Factions::NVA => {
                space.get_forces(Forces::NvaBase).unwrap()
                    + space.get_forces(Forces::TunneledNvaBase).unwrap()
            }
            Factions::VC => {
                space.get_forces(Forces::VcBase).unwrap()
                    + space.get_forces(Forces::TunneledVcBase).unwrap()
            }
            _ => panic!(
                "Calculate number of a factions bases in space not implemented for faction {:?}",
                faction
            ),
        }
    };

    Ok(number_of_a_factions_bases_in_space)
}
