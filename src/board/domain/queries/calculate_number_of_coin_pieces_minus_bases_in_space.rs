use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::forces::Forces;

pub fn calculate_number_of_coin_pieces_minus_bases_in_space(
    occupable_space: &Spaces,
) -> Result<u8, String> {
    Ok(occupable_space.get_forces(Forces::ActiveArvnRanger)?
        + occupable_space.get_forces(Forces::ActiveUsIrregular)?
        + occupable_space.get_forces(Forces::ArvnPolice)?
        + occupable_space.get_forces(Forces::ArvnTroop)?
        + occupable_space.get_forces(Forces::UndergroundArvnRanger)?
        + occupable_space.get_forces(Forces::UndergroundUsIrregular)?
        + occupable_space.get_forces(Forces::UsTroop)?)
}
