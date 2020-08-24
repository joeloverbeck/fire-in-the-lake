use board::domain::queries::board_level_queries::is_there_a_minimum_number_of_a_specific_force_in_any_adjacent_space::is_there_a_minimum_number_of_a_specific_force_in_any_adjacent_space;
use board::domain::queries::space_level_queries::is_there_a_big_enough_concentration_of_coin_pieces_for_bombardment::is_there_a_big_enough_concentration_of_coin_pieces_for_bombardment;
use board::domain::space::Spaces;
use game_definitions::terrain_types::TerrainTypes;
use board::domain::queries::space_level_queries::calculate_number_of_faction_cubes_in_space::calculate_number_of_faction_cubes_in_space;
use board::domain::queries::calculate_number_of_forces_of_a_particular_faction_in_space::calculate_number_of_forces_of_a_particular_faction_in_space;
use board::domain::queries::space_level_queries::can_attack_remove_a_number_of_enemies_in_space::can_attack_remove_a_number_of_enemies_in_space;
use board::domain::queries::board_level_queries::would_marching_a_particular_force_into_a_space_turn_it_into_nva_control::would_marching_a_particular_force_into_a_space_turn_it_into_nva_control;
use board::domain::queries::space_level_queries::does_space_have_support::does_space_have_support;
use board::domain::board::Board;
use board::domain::queries::are_there_any_of_a_particular_force_in_space::are_there_any_of_a_particular_force_in_space;
use board::domain::queries::calculate_number_of_coin_bases::calculate_number_of_coin_bases;
use board::domain::queries::calculate_number_of_coin_cubes_in_space::calculate_number_of_coin_cubes_in_space;
use board::domain::queries::can_attack_remove_base_in_space::can_attack_remove_base_in_space;
use board::domain::space::Space;
use game_definitions::control_types::ControlTypes;
use game_definitions::faction_groups::FactionGroups;
use game_definitions::factions::Factions;
use game_definitions::forces::Forces;
use game_definitions::geographic_areas::GeographicAreas;
use game_definitions::space_identifiers::SpaceIdentifiers;
use game_definitions::support_levels::SupportLevels;

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

        let mut can_attack_remove_minimum_number_of_enemies = false;

        for (_, occupable_space) in board.get_occupable_spaces()?.iter() {
            can_attack_remove_minimum_number_of_enemies =
                can_attack_remove_a_number_of_enemies_in_space(
                    faction,
                    occupable_space,
                    minimum_number_of_enemies_to_remove,
                )?;

            if can_attack_remove_minimum_number_of_enemies {
                break;
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

    pub fn is_there_a_specific_force_in_a_terrain_type(
        &self,
        force: Forces,
        terrain_type: TerrainTypes,
        board: &Board,
    ) -> Result<bool, String> {
        Ok(board
            .get_occupable_spaces()?
            .iter()
            .any(|(_, occupable_space)| {
                occupable_space.is_habitable().unwrap()
                    && occupable_space.get_terrain_type().unwrap() == &terrain_type
                    && occupable_space.get_forces(force).unwrap() > 0
            }))
    }

    pub fn is_there_a_specific_force_in_any_province(
        &self,
        force: Forces,
        board: &Board,
    ) -> Result<bool, String> {
        // Provinces are either Highlands, Lowlands or Jungles.

        Ok(!self
            .get_space_identifiers_with_a_particular_force_and_terrain_type(
                force,
                TerrainTypes::Lowland,
                board,
            )?
            .is_empty()
            || !self
                .get_space_identifiers_with_a_particular_force_and_terrain_type(
                    force,
                    TerrainTypes::Highland,
                    board,
                )?
                .is_empty()
            || !self
                .get_space_identifiers_with_a_particular_force_and_terrain_type(
                    force,
                    TerrainTypes::Jungle,
                    board,
                )?
                .is_empty())
    }

    pub fn is_there_any_number_of_a_specific_force_in_a_geographic_area(
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

    pub fn are_there_any_cubes_of_a_faction_anywhere(
        &self,
        faction: Factions,
        board: &Board,
    ) -> Result<bool, String> {
        Ok(board
            .get_occupable_spaces()?
            .iter()
            .map(|(_, occupable_space)| {
                calculate_number_of_faction_cubes_in_space(faction, occupable_space).unwrap()
            })
            .sum::<u8>()
            > 0)
    }

    pub fn are_there_any_forces_of_a_faction_anywhere(
        &self,
        faction: Factions,
        board: &Board,
    ) -> Result<bool, String> {
        let sum: u8 = board
            .get_occupable_spaces()?
            .iter()
            .map(|(_, occupable_space)| {
                calculate_number_of_forces_of_a_particular_faction_in_space(
                    faction,
                    occupable_space,
                )
                .unwrap()
            })
            .sum();

        Ok(sum > 0)
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

    pub fn get_space_identifiers_with_a_particular_force_and_terrain_type(
        &self,
        force: Forces,
        terrain_type: TerrainTypes,
        board: &'a Board,
    ) -> Result<Vec<&'a SpaceIdentifiers>, String> {
        Ok(board
            .get_occupable_spaces()?
            .iter()
            .filter(|(_, occupable_space)| occupable_space.is_habitable().unwrap())
            .filter(|(_, occupable_space)| {
                occupable_space.get_terrain_type().unwrap() == &terrain_type
            })
            .filter(|(_, occupable_space)| {
                are_there_any_of_a_particular_force_in_space(force, &occupable_space).unwrap()
            })
            .map(|(space_identifier, _)| space_identifier)
            .collect::<Vec<&SpaceIdentifiers>>())
    }

    pub fn are_there_any_us_irregulars_on_laos_or_cambodia(
        &self,
        board: &Board,
    ) -> Result<bool, String> {
        Ok(
            (self.is_there_any_number_of_a_specific_force_in_a_geographic_area(
                Forces::UndergroundUsIrregular,
                &GeographicAreas::Laos,
                board,
            )? || self.is_there_any_number_of_a_specific_force_in_a_geographic_area(
                Forces::ActiveUsIrregular,
                &GeographicAreas::Laos,
                board,
            )?) || (self.is_there_any_number_of_a_specific_force_in_a_geographic_area(
                Forces::UndergroundUsIrregular,
                &GeographicAreas::Cambodia,
                board,
            )? || self.is_there_any_number_of_a_specific_force_in_a_geographic_area(
                Forces::ActiveUsIrregular,
                &GeographicAreas::Cambodia,
                board,
            )?),
        )
    }

    pub fn are_there_any_casualties(&self, board: &Board) -> Result<bool, String> {
        // NOTE: this works under the assumption that only US forces go to Casualties.

        Ok(
            board.get_forces_in_casualties(Forces::UndergroundUsIrregular)? > 0
                || board.get_forces_in_casualties(Forces::ActiveUsIrregular)? > 0
                || board.get_forces_in_casualties(Forces::UsTroop)? > 0
                || board.get_forces_in_casualties(Forces::UsBase)? > 0,
        )
    }

    pub fn does_space_identifier_have_support(
        &self,
        space_identifier: SpaceIdentifiers,
        board: &Board,
    ) -> Result<bool, String> {
        let corresponding_space = board.get_space(space_identifier)?;

        Ok(does_space_have_support(&corresponding_space)?)
    }

    pub fn are_there_any_forces_of_a_faction_available(
        &self,
        faction: Factions,
        board: &Board,
    ) -> Result<bool, String> {
        match faction {
            Factions::US => Ok(board.get_forces_available(Forces::UsTroop)? > 0
                || board.get_forces_available(Forces::ActiveUsIrregular)? > 0
                || board.get_forces_available(Forces::UndergroundUsIrregular)? > 0
                || board.get_forces_available(Forces::UsBase)? > 0),
            _ => panic!("Not implemented for {:?}", faction),
        }
    }

    pub fn get_spaces_with_enough_concentration_of_coin_troops_to_bombard_but_only_nva_troops_adjacent(
        &self,
        board: &'a Board,
    ) -> Result<Vec<&'a Spaces>, String> {
        let bombardment_targets_with_nva_troops_present: Vec<&Spaces> = self
            .get_spaces_with_enough_concentration_of_coin_troops_to_bombard_and_nva_troops_present(
                board,
            )?;

        Ok(board
            .get_occupable_spaces()?
            .iter()
            .filter(|(_, occupable_space)| {
                !bombardment_targets_with_nva_troops_present
                    .iter()
                    .any(|other_occupable_spaces| {
                        other_occupable_spaces.get_space_identifier().unwrap()
                            == occupable_space.get_space_identifier().unwrap()
                    })
            })
            .filter(|(_, occupable_space)| {
                is_there_a_big_enough_concentration_of_coin_pieces_for_bombardment(occupable_space)
                    .unwrap()
            })
            .filter(|(_, occupable_space)| {
                is_there_a_minimum_number_of_a_specific_force_in_any_adjacent_space(
                    Forces::NvaTroop,
                    3,
                    occupable_space,
                    board,
                )
                .unwrap()
            })
            .map(|(_, occupable_space)| occupable_space)
            .collect::<Vec<&Spaces>>())
    }

    pub fn get_spaces_with_enough_concentration_of_coin_troops_to_bombard_and_nva_troops_present(
        &self,
        board: &'a Board,
    ) -> Result<Vec<&'a Spaces>, String> {
        // Rules: Bombard 2 spaces (in or adjacent to 1 or more spaces with at least 3 NVA Troops, and with either at least 3 COIN
        // troops or a COIN Base and at least 1 COIN Troop).

        Ok(board
            .get_occupable_spaces()?
            .iter()
            .filter(|(_, occupable_space)| {
                is_there_a_big_enough_concentration_of_coin_pieces_for_bombardment(occupable_space)
                    .unwrap()
            })
            .filter(|(_, occupable_space)| {
                occupable_space.get_forces(Forces::NvaTroop).unwrap() >= 3
            })
            .map(|(_, occupable_space)| occupable_space)
            .collect::<Vec<&Spaces>>())
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
            would_marching_turn_space_into_nva_control =
                would_marching_a_particular_force_into_a_space_turn_it_into_nva_control(
                    force,
                    occupable_space,
                    &space_identifiers,
                    board,
                )?;

            if would_marching_turn_space_into_nva_control {
                break;
            }
        }

        Ok(would_marching_turn_space_into_nva_control)
    }

    pub fn get_spaces_nva_can_terror_and_have_minimum_population(
        &self,
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
            sut.is_there_any_number_of_a_specific_force_in_a_geographic_area(
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
            sut.is_there_any_number_of_a_specific_force_in_a_geographic_area(
                Forces::UndergroundUsIrregular,
                &GeographicAreas::NorthVietnam,
                &board
            )?,
            false
        );

        Ok(())
    }

    #[test]
    fn test_if_asks_about_a_force_being_in_a_terrain_type_it_says_no_if_that_force_is_not_in_that_terrain_type(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::TheFishhook)?;

        let sut = QueriesController::new();

        assert!(!sut.is_there_a_specific_force_in_a_terrain_type(
            Forces::NvaTroop,
            TerrainTypes::Highland,
            &board
        )?);

        Ok(())
    }

    #[test]
    fn test_if_asks_about_a_force_being_in_a_terrain_type_it_says_there_is_if_that_force_is_in_such_a_terrain_type(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::TheFishhook)?;

        let sut = QueriesController::new();

        assert!(sut.is_there_a_specific_force_in_a_terrain_type(
            Forces::NvaTroop,
            TerrainTypes::Jungle,
            &board
        )?);

        Ok(())
    }

    #[test]
    fn test_can_find_if_there_is_a_specific_force_in_a_province() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::TheFishhook)?;

        let sut = QueriesController::new();

        assert!(sut.is_there_a_specific_force_in_any_province(Forces::NvaTroop, &board)?);

        Ok(())
    }

    #[test]
    fn test_wont_say_there_is_a_force_in_a_province_if_it_is_in_a_city() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::NvaTroop, 3, SpaceIdentifiers::Saigon)?;

        let sut = QueriesController::new();

        assert!(!sut.is_there_a_specific_force_in_any_province(Forces::NvaTroop, &board)?);

        Ok(())
    }

    #[test]
    fn test_wont_say_there_are_cubes_of_a_specific_faction_if_the_forces_present_arent_cubes(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(
            Forces::UndergroundUsIrregular,
            1,
            SpaceIdentifiers::PleikuDarlac,
        )?;

        let sut = QueriesController::new();

        assert!(!sut.are_there_any_cubes_of_a_faction_anywhere(Factions::US, &board)?);

        Ok(())
    }

    #[test]
    fn test_will_say_there_are_cubes_of_a_faction_somewhere_if_there_are_indeed_any_cubes_of_that_faction(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::PleikuDarlac)?;

        let sut = QueriesController::new();

        assert!(sut.are_there_any_cubes_of_a_faction_anywhere(Factions::US, &board)?);

        Ok(())
    }

    #[test]
    fn test_when_asking_to_get_space_identifiers_with_a_specific_force_and_terrain_type_it_will_return_an_empty_vec_if_there_are_no_coincidences(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::TheParrotsBeak)?;

        let sut = QueriesController::new();

        let space_identifiers = sut
            .get_space_identifiers_with_a_particular_force_and_terrain_type(
                Forces::UsTroop,
                TerrainTypes::Highland,
                &board,
            )?;

        assert_eq!(space_identifiers.len(), 0);

        Ok(())
    }

    #[test]
    fn test_when_asking_to_get_space_identifiers_with_a_specific_force_and_terrain_type_it_will_return_expected_space_identifier_if_there_are_coincidences(
    ) -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::UsTroop, 1, SpaceIdentifiers::TheParrotsBeak)?;

        let sut = QueriesController::new();

        let space_identifiers = sut
            .get_space_identifiers_with_a_particular_force_and_terrain_type(
                Forces::UsTroop,
                TerrainTypes::Jungle,
                &board,
            )?;

        assert_eq!(space_identifiers.len(), 1);
        assert_eq!(space_identifiers[0], &SpaceIdentifiers::TheParrotsBeak);

        Ok(())
    }

    #[test]
    fn test_wont_say_there_are_us_irregulars_in_cambodia_if_there_arent_any() -> Result<(), String>
    {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ActiveUsIrregular, 1, SpaceIdentifiers::PleikuDarlac)?;

        let sut = QueriesController::new();

        assert!(!sut.are_there_any_us_irregulars_on_laos_or_cambodia(&board)?);

        Ok(())
    }

    #[test]
    fn test_will_say_there_are_us_irregulars_in_cambodia_if_there_are() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(
            Forces::ActiveUsIrregular,
            1,
            SpaceIdentifiers::TheParrotsBeak,
        )?;

        let sut = QueriesController::new();

        assert!(sut.are_there_any_us_irregulars_on_laos_or_cambodia(&board)?);

        Ok(())
    }

    #[test]
    fn test_will_say_there_are_us_irregulars_in_laos_if_there_are() -> Result<(), String> {
        let mut board = Board::new();

        board.set_forces_in_space(Forces::ActiveUsIrregular, 1, SpaceIdentifiers::CentralLaos)?;

        let sut = QueriesController::new();

        assert!(sut.are_there_any_us_irregulars_on_laos_or_cambodia(&board)?);

        Ok(())
    }

    #[test]
    fn test_wont_say_there_are_casualties_if_there_are_none() -> Result<(), String> {
        let board = Board::new();

        let sut = QueriesController::new();

        assert!(!sut.are_there_any_casualties(&board)?);

        Ok(())
    }

    #[test]
    fn test_will_say_there_are_casualties_if_there_are() -> Result<(), String> {
        let mut board = Board::new();

        board.increase_forces_in_space(&Forces::UsTroop, SpaceIdentifiers::Casualties, 1)?;

        let sut = QueriesController::new();

        assert!(sut.are_there_any_casualties(&board)?);

        Ok(())
    }

    #[test]
    fn test_wont_say_space_has_support_if_it_doesnt_have() -> Result<(), String> {
        let mut board = Board::new();

        board.set_support_level_of_space(
            SupportLevels::PassiveOpposition,
            SpaceIdentifiers::PleikuDarlac,
        )?;

        let sut = QueriesController::new();

        assert!(!sut.does_space_identifier_have_support(SpaceIdentifiers::PleikuDarlac, &board)?);

        Ok(())
    }

    #[test]
    fn test_will_say_space_has_support_if_it_does() -> Result<(), String> {
        let mut board = Board::new();

        board.set_support_level_of_space(
            SupportLevels::PassiveSupport,
            SpaceIdentifiers::PleikuDarlac,
        )?;

        let sut = QueriesController::new();

        assert!(sut.does_space_identifier_have_support(SpaceIdentifiers::PleikuDarlac, &board)?);

        Ok(())
    }

    #[test]
    fn test_wont_say_there_are_us_forces_available_if_there_are_none() -> Result<(), String> {
        let board = Board::new();

        let sut = QueriesController::new();

        assert!(!sut.are_there_any_forces_of_a_faction_available(Factions::US, &board)?);

        Ok(())
    }

    #[test]
    fn test_will_say_there_are_us_forces_available_if_there_are() -> Result<(), String> {
        let mut board = Board::new();

        board.increase_forces_in_space(
            &Forces::UndergroundUsIrregular,
            SpaceIdentifiers::Available,
            1,
        )?;

        let sut = QueriesController::new();

        assert!(sut.are_there_any_forces_of_a_faction_available(Factions::US, &board)?);

        Ok(())
    }
}
