use board::domain::board::Board;
use board::domain::queries::are_there_any_of_a_particular_force_in_space::are_there_any_of_a_particular_force_in_space;
use board::domain::queries::calculate_number_of_a_particular_force_in_space::calculate_number_of_a_particular_force_in_space;
use board::domain::queries::calculate_number_of_coin_bases::calculate_number_of_coin_bases;
use board::domain::queries::calculate_number_of_coin_cubes_in_space::calculate_number_of_coin_cubes_in_space;
use board::domain::queries::calculate_number_of_coin_pieces_minus_bases_in_space::calculate_number_of_coin_pieces_minus_bases_in_space;
use board::domain::queries::can_attack_remove_base_in_space::can_attack_remove_base_in_space;
use board::domain::queries::will_moving_in_number_of_nva_forces_turn_space_into_nva_control::will_moving_in_number_of_nva_forces_turn_space_into_nva_control;
use board::domain::space::Space;
use game_definitions::control_types::ControlTypes;
use game_definitions::faction_groups::FactionGroups;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;

#[derive(Debug)]
pub struct QueriesController {}

impl Default for QueriesController {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> QueriesController {
    pub fn new() -> QueriesController {
        QueriesController {}
    }

    pub fn can_attack_remove_a_number_of_enemies(
        &self,
        faction: &Factions,
        minimum_number_of_enemies_to_remove: u8,
        board: &Board,
    ) -> Result<bool, String> {
        // Rules: roll a die in a space and if the throw is less than the number of the executing faction's
        // guerrillas, remove up to 2 enemy pieces (only remove bases when there are no other coin forces in the space).

        // First off, if the minimum number of enemies to remove with an attack is bigger than two, the anwer
        // is no.
        if minimum_number_of_enemies_to_remove > 2 {
            return Ok(false);
        }

        let can_attack_remove_minimum_number_of_enemies = false;

        for (_, occupable_space) in board.get_occupable_spaces()?.iter() {
            if faction == &Factions::NVA
                && (calculate_number_of_a_particular_force_in_space(
                    Forces::ActiveNvaGuerrilla,
                    &occupable_space,
                )? + calculate_number_of_a_particular_force_in_space(
                    Forces::UndergroundNvaGuerrilla,
                    &occupable_space,
                )?) == 0
            {
                // Can't kill anybody here with any dice throw.
                continue;
            }

            if faction == &Factions::VC
                && (calculate_number_of_a_particular_force_in_space(
                    Forces::ActiveVcGuerrilla,
                    &occupable_space,
                )? + calculate_number_of_a_particular_force_in_space(
                    Forces::UndergroundVcGuerrilla,
                    &occupable_space,
                )?) == 0
            {
                // Can't kill anybody here with any dice throw.
                continue;
            }

            // We know that it could kill up to 2 in a successful throw, but there must be enemy pieces here.
            if calculate_number_of_coin_pieces_minus_bases_in_space(&occupable_space)?
                >= minimum_number_of_enemies_to_remove
            {
                return Ok(true);
            }
        }

        Ok(can_attack_remove_minimum_number_of_enemies)
    }

    pub fn can_attack_remove_base(
        &self,
        faction: &Factions,
        board: &Board,
    ) -> Result<bool, String> {
        // Rules: an Attack can remove an US or ARVN base ONLY when there are no other pieces
        // of EITHER faction in the space.
        // 1) Roll a die: if the roll is less than or equal to the number of the executing's
        // faction Guerrillas there (active or not), remove UP TO 2 enemy pieces of the executing faction's choice.
        // 2) NVA may instead remove 1 enemy piece per 2 NVA Troops there (rounded down, without activating the Guerrillas.)
        //    The removed pieces may belong to different factions, and can be Underground Special Forces.

        for (_, occupable_space) in board.get_occupable_spaces()?.iter() {
            let can_attack_remove_base_in_space =
                can_attack_remove_base_in_space(occupable_space, faction)?;

            if can_attack_remove_base_in_space {
                return Ok(can_attack_remove_base_in_space);
            }
        }

        Ok(false)
    }

    pub fn is_there_a_faction_group_base_in_a_province_with_less_or_equal_cubes_of_that_group(
        &self,
        faction_group: &FactionGroups,
        max_number_of_cubes: u8,
        board: &Board,
    ) -> Result<bool, String> {
        Ok(board
            .get_occupable_spaces()?
            .iter()
            .any(|(_, occupable_space)| {
                // If faction group is Coin, those cubes would be UsTroops, ArvnTroops or ArvnPolice.
                // If faction group is Insurgent, those cubes would just be NvaTroops
                if faction_group == &FactionGroups::Coin {
                    if calculate_number_of_coin_bases(&occupable_space).unwrap() > 0
                        && calculate_number_of_coin_cubes_in_space(&occupable_space).unwrap()
                            <= max_number_of_cubes
                    {
                        return true;
                    }

                    false
                } else {
                    panic!("Case not handled when the faction group is not Coin.");
                }
            }))
    }

    pub fn is_there_a_specific_force_anywhere(
        &self,
        force: Forces,
        board: &Board,
    ) -> Result<bool, String> {
        Ok(board
            .get_occupable_spaces()?
            .iter()
            .any(|(_, occupable_space)| occupable_space.get_forces(force).unwrap() > 0))
    }

    pub fn is_there_any_number_of_a_particular_force_in_a_geographic_area(
        &self,
        force: Forces,
        geographic_area: &GeographicAreas,
        board: &Board,
    ) -> Result<bool, String> {
        // We go through all the stored spaces, and if any of them are in the geographic area, there just needs to hold
        // one of the passed force
        Ok(board
            .get_occupable_spaces()?
            .iter()
            .filter(|(_, occupable_space)| {
                occupable_space.get_geographic_area().unwrap() == geographic_area
            })
            .any(|(_, occupable_space)| occupable_space.get_forces(force).unwrap() > 0))
    }

    pub fn is_any_space_at_a_specific_control_type_in_a_geographic_area(
        &self,
        control_type: &ControlTypes,
        geographic_area: &GeographicAreas,
        board: &Board,
    ) -> Result<bool, String> {
        Ok(board
            .get_occupable_spaces()?
            .iter()
            .any(|(_, occupable_space)| {
                occupable_space.get_geographic_area().unwrap() == geographic_area
                    && occupable_space.get_control_type().unwrap() == control_type
            }))
    }

    pub fn are_there_any_forces_of_a_faction_anywhere(
        &self,
        faction: Factions,
        board: &Board,
    ) -> Result<bool, String> {
        if faction == Factions::US {
            let sum: u8 = board
                .get_occupable_spaces()?
                .iter()
                .map(|(_, occupable_space)| {
                    occupable_space.get_forces(Forces::UsTroop).unwrap()
                        + occupable_space
                            .get_forces(Forces::ActiveUsIrregular)
                            .unwrap()
                        + occupable_space
                            .get_forces(Forces::UndergroundUsIrregular)
                            .unwrap()
                        + occupable_space.get_forces(Forces::UsBase).unwrap()
                })
                .sum();

            Ok(sum > 0)
        } else {
            panic!("Not implemented for faction {:?}", faction);
        }
    }

    pub fn get_space_identifiers_with_a_particular_force(
        &self,
        force: Forces,
        board: &'a Board,
    ) -> Result<Vec<&'a SpaceIdentifiers>, String> {
        Ok(board
            .get_occupable_spaces()?
            .iter()
            .filter(|(_, occupable_space)| {
                are_there_any_of_a_particular_force_in_space(force, &occupable_space).unwrap()
            })
            .map(|(space_identifier, _)| space_identifier)
            .collect::<Vec<&SpaceIdentifiers>>())
    }

    pub fn would_marching_a_particular_force_into_space_identifiers_turn_any_into_nva_control(
        &self,
        force: Forces,
        space_identifiers: Vec<&SpaceIdentifiers>,
        board: &Board,
    ) -> Result<bool, String> {
        let mut would_marching_turn_space_into_nva_control = false;

        // This is a complicated one. For *all* spaces on the board, must look at which have any of the space identifiers
        // as adjacent. Then, in the checked space, must see how many of the passed force they have. Then we need to check if
        // adding those to the space corresponding to the space identifier would turn it into NvaControl.
        for (_, occupable_space) in board.get_occupable_spaces()?.iter() {
            for &space_identifier in &space_identifiers {
                if !occupable_space.is_adjacent_to_space(*space_identifier)? {
                    continue;
                }

                // The passed space_identifier is adjacent to the current occupable space
                // we are looking at in the board. We need to check if there are forces of the
                // needed type to march into the passed space_identifier.
                if !are_there_any_of_a_particular_force_in_space(force, &occupable_space)? {
                    continue;
                }

                let number_of_particular_force_in_space = occupable_space.get_forces(force)?;

                // There are forces of the needed type. We need another check, though: if the space_identifier
                // is already NvaControl, it would obviously not change to NvaControl
                let corresponding_destination_space = board.get_space(*space_identifier)?;

                if corresponding_destination_space.get_control_type()? == &ControlTypes::Nva {
                    continue;
                }

                // Final check:
                if will_moving_in_number_of_nva_forces_turn_space_into_nva_control(
                    number_of_particular_force_in_space,
                    corresponding_destination_space,
                )? {
                    would_marching_turn_space_into_nva_control = true;
                    break;
                }
            }
        }

        Ok(would_marching_turn_space_into_nva_control)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use game_definitions::control_types::ControlTypes;
    use game_definitions::factions::Factions;
    use game_definitions::forces::Forces;
    use game_definitions::geographic_areas::GeographicAreas;
    use game_definitions::space_identifiers::SpaceIdentifiers;
    use players::domain::decision::Decision;
    use scenario_building::controllers::scenario_building_controller::ScenarioBuildingController;
    use user_interface::controllers::display_controller_trait::DisplayControllerTrait;

    struct FakeDisplayController {}

    impl FakeDisplayController {
        pub fn new() -> FakeDisplayController {
            FakeDisplayController {}
        }
    }

    impl DisplayControllerTrait for FakeDisplayController {
        fn write_announcement(&self, _text: &str) -> Result<(), String> {
            Ok(())
        }
        fn write_information_for_decision(
            &self,
            _decision: &Decision,
            _faction: &Factions,
        ) -> Result<(), String> {
            Ok(())
        }
        fn write_instructions_for_decision(
            &self,
            _decision: &Decision,
            _faction: &Factions,
        ) -> Result<(), String> {
            Ok(())
        }
        fn write_instruction(&self, _text: &str) -> Result<(), String> {
            Ok(())
        }
        fn write_information(&self, _text: &str) -> Result<(), String> {
            Ok(())
        }
        fn write_section(&self, _text: &str) -> Result<(), String> {
            Ok(())
        }
        fn write_alert(&self, _text: &str) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_can_query_whether_attack_for_nva_can_remove_base() -> Result<(), String> {
        let fake_display_controller = FakeDisplayController::new();

        let board =
            ScenarioBuildingController::new().build_full_scenario(&fake_display_controller)?;

        let queries_controller = QueriesController::new();

        assert_eq!(
            queries_controller.can_attack_remove_base(&Factions::NVA, &board)?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_can_query_whether_attack_can_remove_a_number_of_enemies() -> Result<(), String> {
        let fake_display_controller = FakeDisplayController::new();

        let board =
            ScenarioBuildingController::new().build_full_scenario(&fake_display_controller)?;

        let queries_controller = QueriesController::new();

        assert_eq!(
            queries_controller.can_attack_remove_a_number_of_enemies(&Factions::NVA, 2, &board)?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_can_query_whether_there_is_a_base_of_a_faction_ground_in_a_province_with_less_or_equal_number_of_that_groups_cubes(
    ) -> Result<(), String> {
        // Regarding the original event: It's only effective in its unshaded part if there's a COIN base in a province with less or equal
        // COIN "cubes" than a number specifically (US to Casualties).
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::ArvnPolice, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::Saigon)?;

        let sut = QueriesController::new();

        assert_eq!(
            sut.is_there_a_faction_group_base_in_a_province_with_less_or_equal_cubes_of_that_group(
                &FactionGroups::Coin,
                2,
                &board
            )?,
            true
        );

        Ok(())
    }

    #[test]
    fn test_query_to_whether_theres_a_coin_base_in_a_space_with_less_than_a_number_fails_if_theres_a_higher_number(
    ) -> Result<(), String> {
        // Regarding the original event: It's only effective in its unshaded part if there's a COIN base in a province with less or equal
        // COIN "cubes" than a number specifically (US to Casualties).
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::ArvnPolice, 2, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::Saigon)?;

        let sut = QueriesController::new();

        assert_eq!(
            sut.is_there_a_faction_group_base_in_a_province_with_less_or_equal_cubes_of_that_group(
                &FactionGroups::Coin,
                2,
                &board
            )?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_an_occupable_space_in_a_geographical_area_with_a_certain_control_returns_true(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_control_type_of_space(ControlTypes::Nva, SpaceIdentifiers::TheParrotsBeak)?;

        let sut = QueriesController::new();

        assert_eq!(
            sut.is_any_space_at_a_specific_control_type_in_a_geographic_area(
                &ControlTypes::Nva,
                &GeographicAreas::Cambodia,
                &board
            )?,
            true
        );

        Ok(())
    }

    #[test]
    fn test_an_occupable_space_in_a_geographical_area_without_a_certain_control_returns_false(
    ) -> Result<(), String> {
        let board = Board::new();

        let sut = QueriesController::new();

        assert_eq!(
            sut.is_any_space_at_a_specific_control_type_in_a_geographic_area(
                &ControlTypes::Nva,
                &GeographicAreas::NorthVietnam,
                &board
            )?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_can_find_out_if_theres_a_specific_force_anywhere() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ActiveNvaGuerrilla, 1, SpaceIdentifiers::Kontum)?;

        let sut = QueriesController::new();

        assert!(sut.is_there_a_specific_force_anywhere(Forces::ActiveNvaGuerrilla, &board)?);

        Ok(())
    }

    #[test]
    fn test_can_find_out_if_there_are_any_pieces_of_a_faction_anywhere() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::ActiveUsIrregular, 1, SpaceIdentifiers::BinhDinh)?;

        let sut = QueriesController::new();

        assert!(sut.are_there_any_forces_of_a_faction_anywhere(Factions::US, &board)?);

        Ok(())
    }

    #[test]
    fn test_cant_find_any_pieces_of_a_faction_anywhere_if_there_arent_any() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::ArvnPolice, 1, SpaceIdentifiers::BinhDinh)?;

        let sut = QueriesController::new();

        assert_eq!(
            sut.are_there_any_forces_of_a_faction_anywhere(Factions::US, &board)?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_can_get_space_identifiers_of_spaces_that_have_a_certain_force() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::AnLoc)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::QuangDucLongKhanh)?;
        board.set_forces_in_space(Forces::UndergroundArvnRanger, 2, SpaceIdentifiers::QuangNam)?;

        let sut = QueriesController::new();

        let spaces_with_us_bases =
            sut.get_space_identifiers_with_a_particular_force(Forces::UsBase, &board)?;

        assert_eq!(spaces_with_us_bases.len(), 2);
        assert!(spaces_with_us_bases
            .iter()
            .any(|space_identifier| *space_identifier == &SpaceIdentifiers::AnLoc));
        assert!(spaces_with_us_bases
            .iter()
            .any(|space_identifier| *space_identifier == &SpaceIdentifiers::QuangDucLongKhanh));

        Ok(())
    }

    #[test]
    fn test_marching_nva_troops_into_a_space_identifier_would_turn_it_into_nva_control(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 4, SpaceIdentifiers::BinhTuyBinhThuan)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundArvnRanger, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundVcGuerrilla, 1, SpaceIdentifiers::Saigon)?;

        let sut = QueriesController::new();

        assert!(sut
            .would_marching_a_particular_force_into_space_identifiers_turn_any_into_nva_control(
                Forces::NvaTroop,
                vec![&SpaceIdentifiers::Saigon],
                &board
            )?);

        Ok(())
    }

    #[test]
    fn test_marching_too_few_nva_troops_into_a_space_identifier_wouldnt_turn_it_into_nva_control(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::BinhTuyBinhThuan)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundArvnRanger, 1, SpaceIdentifiers::Saigon)?;
        board.set_forces_in_space(Forces::UndergroundVcGuerrilla, 1, SpaceIdentifiers::Saigon)?;

        let sut = QueriesController::new();

        assert_eq!(
            sut.would_marching_a_particular_force_into_space_identifiers_turn_any_into_nva_control(
                Forces::NvaTroop,
                vec![&SpaceIdentifiers::Saigon],
                &board
            )?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_can_figure_out_if_there_are_any_of_a_particular_force_in_a_geographic_area(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::TheFishhook)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::TheParrotsBeak)?;
        board.set_forces_in_space(
            Forces::UndergroundUsIrregular,
            1,
            SpaceIdentifiers::CentralLaos,
        )?;

        let sut = QueriesController::new();

        assert!(
            sut.is_there_any_number_of_a_particular_force_in_a_geographic_area(
                Forces::UndergroundUsIrregular,
                &GeographicAreas::Laos,
                &board
            )?
        );

        Ok(())
    }

    #[test]
    fn test_can_figure_out_if_there_isnt_any_of_a_particular_force_in_a_geographic_area(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::TheFishhook)?;
        board.set_forces_in_space(Forces::UsBase, 1, SpaceIdentifiers::TheParrotsBeak)?;
        board.set_forces_in_space(
            Forces::UndergroundUsIrregular,
            1,
            SpaceIdentifiers::CentralLaos,
        )?;

        let sut = QueriesController::new();

        assert_eq!(
            sut.is_there_any_number_of_a_particular_force_in_a_geographic_area(
                Forces::UndergroundUsIrregular,
                &GeographicAreas::NorthVietnam,
                &board
            )?,
            false
        );

        Ok(())
    }
}
