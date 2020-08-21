use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;

pub fn calculate_number_of_forces_of_a_particular_faction_in_space(
    faction: Factions,
    occupable_space: &Spaces,
) -> Result<u8, String> {
    match faction {
        Factions::ARVN => Ok(occupable_space.get_forces(Forces::ActiveArvnRanger)?
            + occupable_space.get_forces(Forces::UndergroundArvnRanger)?
            + occupable_space.get_forces(Forces::ArvnBase)?
            + occupable_space.get_forces(Forces::ArvnPolice)?
            + occupable_space.get_forces(Forces::ArvnTroop)?),
        Factions::NVA => Ok(occupable_space.get_forces(Forces::ActiveNvaGuerrilla)?
            + occupable_space.get_forces(Forces::UndergroundNvaGuerrilla)?
            + occupable_space.get_forces(Forces::NvaBase)?
            + occupable_space.get_forces(Forces::NvaTroop)?
            + occupable_space.get_forces(Forces::TunneledNvaBase)?),
        Factions::US => Ok(occupable_space.get_forces(Forces::ActiveUsIrregular)?
            + occupable_space.get_forces(Forces::UndergroundUsIrregular)?
            + occupable_space.get_forces(Forces::UsBase)?
            + occupable_space.get_forces(Forces::UsTroop)?),
        Factions::VC => Ok(occupable_space.get_forces(Forces::ActiveVcGuerrilla)?
            + occupable_space.get_forces(Forces::UndergroundVcGuerrilla)?
            + occupable_space.get_forces(Forces::TunneledVcBase)?
            + occupable_space.get_forces(Forces::VcBase)?),
    }
}
