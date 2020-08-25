use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::faction_groups::FactionGroups;
use game_definitions::forces::Forces;

pub fn calculate_number_of_faction_group_bases_in_space(
    faction_group: FactionGroups,
    occupable_space: &Spaces,
) -> Result<u8, String> {
    match faction_group{
        FactionGroups::Coin => Ok(occupable_space.get_forces(Forces::ArvnBase)?
        + occupable_space.get_forces(Forces::UsBase)?),
        FactionGroups::Insurgent => panic!("Calculate number of faction group bases in space not implemented for faction group {:?}", faction_group)
    }
}
