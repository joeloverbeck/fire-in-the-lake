use board::domain::board::Board;
use board::domain::queries::space_level_queries::can_attack_remove_base_in_space::can_attack_remove_base_in_space;
use game_definitions::factions::Factions;

pub fn can_attack_remove_base(faction: &Factions, board: &Board) -> Result<bool, String> {
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

#[cfg(test)]
mod tests {
    use super::*;
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

        assert_eq!(can_attack_remove_base(&Factions::NVA, &board)?, false);

        Ok(())
    }
}
