use board::domain::board::Board;
use board::domain::calculate_number_of_coin_pieces_minus_bases_in_space::calculate_number_of_coin_pieces_minus_bases_in_space;
use board::domain::calculate_number_of_nva_guerrillas_in_space::calculate_number_of_nva_guerrillas_in_space;
use board::domain::calculate_number_of_vc_guerrillas_in_space::calculate_number_of_vc_guerrillas_in_space;
use board::domain::can_attack_remove_base_in_space::can_attack_remove_base_in_space;
use game_definitions::factions::Factions;

pub struct QueriesController {}

impl Default for QueriesController {
    fn default() -> Self {
        Self::new()
    }
}

impl QueriesController {
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
                && calculate_number_of_nva_guerrillas_in_space(&occupable_space)? == 0
            {
                // Can't kill anybody here with any dice throw.
                continue;
            }

            if faction == &Factions::VC
                && calculate_number_of_vc_guerrillas_in_space(&occupable_space)? == 0
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
}
