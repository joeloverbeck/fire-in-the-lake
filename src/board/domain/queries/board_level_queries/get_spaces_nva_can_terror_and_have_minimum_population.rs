use board::domain::board::Board;
use board::domain::space::Space;
use board::domain::space::Spaces;
use game_definitions::forces::Forces;
use game_definitions::support_levels::SupportLevels;

pub fn get_spaces_nva_can_terror_and_have_minimum_population<'a>(
    minimum_population: u8,
    board: &'a Board,
) -> Result<Vec<&'a Spaces>, String> {
    Ok(board
        .get_occupable_spaces()?
        .iter()
        .filter(|(_, occupable_space)| occupable_space.is_habitable().unwrap())
        .filter(|(_, occupable_space)| {
            occupable_space.get_support_level().unwrap() == &SupportLevels::PassiveSupport
                || occupable_space.get_support_level().unwrap() == &SupportLevels::ActiveSupport
        })
        .filter(|(_, occupable_space)| {
            occupable_space
                .get_forces(Forces::UndergroundNvaGuerrilla)
                .unwrap()
                > 0
                || occupable_space.get_forces(Forces::NvaTroop).unwrap() > 0
        })
        .filter(|(_, occupable_space)| {
            occupable_space.get_population().unwrap() >= minimum_population
        })
        .map(|(_, occupable_space)| occupable_space)
        .collect::<Vec<&Spaces>>())
}
