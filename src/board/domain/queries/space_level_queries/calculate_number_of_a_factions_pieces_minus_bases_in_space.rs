use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;

pub fn calculate_number_of_a_factions_pieces_minus_bases_in_space(
    faction: Factions,
    space: &Spaces,
) -> Result<u8, String> {
    match faction {
        Factions::NVA => {
            Ok(space.get_forces(Forces::UndergroundNvaGuerrilla)? + space.get_forces(Forces::ActiveNvaGuerrilla)? + space.get_forces(Forces::NvaTroop)?)
        },
        Factions::VC => {
          Ok(space.get_forces(Forces::UndergroundVcGuerrilla)? + space.get_forces(Forces::ActiveVcGuerrilla)?)
        },
        _ => panic!("Calculating the number of a faction's pieces minus bases in space not implemented for {:?}", faction)
    }
}
