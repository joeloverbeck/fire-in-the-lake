use board::domain::queries::space_level_queries::calculate_number_of_a_factions_special_rangers_in_space::calculate_number_of_a_factions_special_rangers_in_space;
use game_definitions::constants::MINIMUM_NUMBER_OF_NVA_TROOPS_THAT_SHOULD_BE_PLACED_THROUGH_INFILTRATE_TO_GO_AHEAD_WITH_IT;
use board::domain::queries::space_level_queries::calculate_number_of_a_factions_bases_in_space::calculate_number_of_a_factions_bases_in_space;
use board::domain::queries::space_level_queries::are_there_faction_bases_in_space::are_there_faction_bases_in_space;
use board::domain::board::Board;
use game_definitions::faction_stats::FactionStats;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use std::cmp::Ordering;
use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn would_placing_nva_troops_anywhere_through_infiltrate_place_enough(
    times_it_can_infiltrate: u8,
    board: &Board,
) -> Result<bool, String> {
    // In spaces where there is a NVA Base, NVA can place NVA Troops up to the trail value plus the number of NVA Bases there. Then it can replace
    // any NVA Guerrillas desired 1 for 1 with added NVA Troops.

    // First off, if there aren't enough NVA Troops available, this can't work.
    if board
        .get_forces_in_space(Forces::UndergroundNvaGuerrilla, SpaceIdentifiers::Available)
        .unwrap()
        < MINIMUM_NUMBER_OF_NVA_TROOPS_THAT_SHOULD_BE_PLACED_THROUGH_INFILTRATE_TO_GO_AHEAD_WITH_IT
    {
        return Ok(false);
    }

    let mut number_of_nva_troops_that_can_be_placed_per_space = board
        .get_occupable_spaces()?
        .iter()
        .filter(|(_, occupable_space)| {
            are_there_faction_bases_in_space(Factions::NVA, occupable_space).unwrap()
        })
        .map(|(_, occupable_space)| {
            let number_of_nva_guerrillas_in_space =
                calculate_number_of_a_factions_special_rangers_in_space(
                    Factions::NVA,
                    occupable_space,
                )
                .unwrap();

            let number_of_nva_guerrillas_that_can_be_replaced = {
                if number_of_nva_guerrillas_in_space >= 2 {
                    number_of_nva_guerrillas_in_space - 2
                } else {
                    0
                }
            };

            board.get_faction_stat(FactionStats::TheTrail).unwrap()
                + calculate_number_of_a_factions_bases_in_space(Factions::NVA, occupable_space)
                    .unwrap()
                + number_of_nva_guerrillas_that_can_be_replaced
        })
        .collect::<Vec<u8>>();

    number_of_nva_troops_that_can_be_placed_per_space
        .sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let mut_times_it_can_infiltrate: u8 = {
        if times_it_can_infiltrate > number_of_nva_troops_that_can_be_placed_per_space.len() as u8 {
            number_of_nva_troops_that_can_be_placed_per_space.len() as u8
        } else {
            times_it_can_infiltrate
        }
    };

    match mut_times_it_can_infiltrate{
        1 => Ok(number_of_nva_troops_that_can_be_placed_per_space[0] >= MINIMUM_NUMBER_OF_NVA_TROOPS_THAT_SHOULD_BE_PLACED_THROUGH_INFILTRATE_TO_GO_AHEAD_WITH_IT),
        2 => Ok(number_of_nva_troops_that_can_be_placed_per_space[0] + number_of_nva_troops_that_can_be_placed_per_space[1] >= MINIMUM_NUMBER_OF_NVA_TROOPS_THAT_SHOULD_BE_PLACED_THROUGH_INFILTRATE_TO_GO_AHEAD_WITH_IT),
        _ => panic!("Couldn't handle infiltrating {:?} times.", times_it_can_infiltrate)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_if_there_arent_any_places_in_the_board_where_placing_troops_through_infiltrate_would_produce_at_least_four_then_asking_returns_false(
    ) -> Result<(), String> {
        let board = Board::new();

        assert_eq!(
            would_placing_nva_troops_anywhere_through_infiltrate_place_enough(1, &board)?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_if_theres_a_place_where_placing_troops_through_infiltrate_would_put_enough_troops_asking_will_return_true(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_faction_stat(FactionStats::TheTrail, 2)?;
        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::AnLoc)?;
        board.set_forces_in_space(Forces::UndergroundNvaGuerrilla, 3, SpaceIdentifiers::AnLoc)?;

        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            6,
            SpaceIdentifiers::Available,
        )?;

        assert_eq!(
            would_placing_nva_troops_anywhere_through_infiltrate_place_enough(1, &board)?,
            true
        );

        Ok(())
    }

    #[test]
    fn test_if_theres_a_place_where_placing_troops_through_infiltrate_would_put_almost_enough_troops_asking_will_return_false(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_faction_stat(FactionStats::TheTrail, 2)?;
        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::AnLoc)?;
        board.set_forces_in_space(Forces::UndergroundNvaGuerrilla, 2, SpaceIdentifiers::AnLoc)?;

        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            6,
            SpaceIdentifiers::Available,
        )?;

        assert_eq!(
            would_placing_nva_troops_anywhere_through_infiltrate_place_enough(1, &board)?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_if_there_are_two_places_combined_where_placing_troops_through_infiltrate_would_put_enough_troops_asking_will_return_true(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_faction_stat(FactionStats::TheTrail, 2)?;
        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::AnLoc)?;

        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::Saigon)?;

        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            6,
            SpaceIdentifiers::Available,
        )?;

        assert_eq!(
            would_placing_nva_troops_anywhere_through_infiltrate_place_enough(2, &board)?,
            true
        );

        Ok(())
    }

    #[test]
    fn test_if_there_are_two_places_combined_where_placing_troops_through_infiltrate_would_put_enough_troops_yet_one_of_them_doesnt_have_a_base_asking_will_return_false(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_faction_stat(FactionStats::TheTrail, 2)?;
        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::AnLoc)?;

        board.set_forces_in_space(Forces::UndergroundNvaGuerrilla, 6, SpaceIdentifiers::Saigon)?;

        board.set_forces_in_space(
            Forces::UndergroundNvaGuerrilla,
            6,
            SpaceIdentifiers::Available,
        )?;

        assert_eq!(
            would_placing_nva_troops_anywhere_through_infiltrate_place_enough(2, &board)?,
            false
        );

        Ok(())
    }
}
