use board::domain::board::Board;
use board::domain::queries::space_level_queries::can_attack_remove_a_number_of_enemies_in_space::can_attack_remove_a_number_of_enemies_in_space;
use game_definitions::factions::Factions;

pub fn can_attack_remove_a_number_of_enemies(
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use game_definitions::factions::Factions;
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
    fn test_can_query_whether_attack_can_remove_a_number_of_enemies() -> Result<(), String> {
        let fake_display_controller = FakeDisplayController::new();

        let board =
            ScenarioBuildingController::new().build_full_scenario(&fake_display_controller)?;

        assert_eq!(
            can_attack_remove_a_number_of_enemies(&Factions::NVA, 2, &board)?,
            false
        );

        Ok(())
    }
}
