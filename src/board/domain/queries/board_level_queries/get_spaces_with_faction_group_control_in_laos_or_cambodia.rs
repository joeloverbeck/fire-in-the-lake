use board::domain::board::Board;
use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::control_types::ControlTypes;
use game_definitions::faction_groups::FactionGroups;
use game_definitions::geographic_areas::GeographicAreas;

pub fn get_spaces_with_faction_group_control_in_laos_or_cambodia<'a>(
    faction_group: FactionGroups,
    board: &'a Board,
) -> Result<Vec<&'a Spaces>, String> {
    Ok(board
        .get_occupable_spaces()?
        .iter()
        .filter(|(_, occupable_space)| occupable_space.get_geographic_area().unwrap() == &GeographicAreas::Laos || occupable_space.get_geographic_area().unwrap() == &GeographicAreas::Cambodia)
        .filter(|(_, occupable_space)| {
            match faction_group{
                FactionGroups::Coin => occupable_space.get_control_type().unwrap() == &ControlTypes::Coin,
                FactionGroups::Insurgent => panic!("Getting spaces with faction group control in Laos or Cambodia not implemented for faction group {:?}", faction_group)
            }
        })
        .map(|(_, occupable_space)| occupable_space)
            .collect::<Vec<&Spaces>>())
}
