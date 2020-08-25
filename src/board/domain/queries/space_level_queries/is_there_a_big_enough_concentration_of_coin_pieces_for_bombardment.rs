use board::domain::queries::space_level_queries::calculate_number_of_coin_cubes_in_space::calculate_number_of_coin_cubes_in_space;
use board::domain::queries::space_level_queries::calculate_number_of_faction_group_bases_in_space::calculate_number_of_faction_group_bases_in_space;
use board::domain::space::Spaces;
use game_definitions::faction_groups::FactionGroups;

pub fn is_there_a_big_enough_concentration_of_coin_pieces_for_bombardment(
    occupable_space: &Spaces,
) -> Result<bool, String> {
    Ok(
        calculate_number_of_coin_cubes_in_space(occupable_space).unwrap() >= 3
            || (calculate_number_of_faction_group_bases_in_space(
                FactionGroups::Coin,
                occupable_space,
            )
            .unwrap()
                >= 1
                && calculate_number_of_coin_cubes_in_space(occupable_space).unwrap() >= 1),
    )
}
