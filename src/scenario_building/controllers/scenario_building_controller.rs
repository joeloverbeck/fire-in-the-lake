use board::controllers::setup_controller::SetupController;
use board::domain::board::Board;
use game_definitions::factions::Factions;
use user_interface::controllers::display_controller::DisplayController;

pub struct ScenarioBuildingController {}

impl Default for ScenarioBuildingController {
    fn default() -> Self {
        Self::new()
    }
}

impl ScenarioBuildingController {
    pub fn new() -> ScenarioBuildingController {
        ScenarioBuildingController {}
    }

    pub fn build_full_scenario(
        &mut self,
        display_controller: &DisplayController,
    ) -> Result<Board, String> {
        display_controller.write_announcement("Full scenario setup")?;

        display_controller.write_section("Faction stats")?;

        let (board, collection_of_instructions) = SetupController::new().setup_full()?;

        for instruction in collection_of_instructions[0].iter() {
            display_controller.write_instruction(instruction.as_str())?;
        }

        display_controller.write_section("Out of Play")?;

        for instruction in collection_of_instructions[1].iter() {
            display_controller.write_instruction(instruction.as_str())?;
        }

        display_controller.write_section("Forces in Spaces")?;

        for instruction in collection_of_instructions[2].iter() {
            display_controller.write_instruction(instruction.as_str())?;
        }

        display_controller.write_section("Control")?;

        for instruction in collection_of_instructions[3].iter() {
            display_controller.write_instruction(instruction.as_str())?;
        }

        display_controller.write_section("Support")?;

        for instruction in collection_of_instructions[4].iter() {
            display_controller.write_instruction(instruction.as_str())?;
        }

        display_controller.write_section("Faction elegibility")?;

        display_controller.write_instruction(
            format!(
                "Set the cylinders of all factions ( {} {} {} {} ) into the Eligible box.",
                Factions::US,
                Factions::ARVN,
                Factions::NVA,
                Factions::VC
            )
            .as_str(),
        )?;

        Ok(board)
    }
}
