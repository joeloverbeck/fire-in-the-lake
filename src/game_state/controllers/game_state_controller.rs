use board::controllers::setup_controller::SetupController;
use user_interface::controllers::user_interface_controller::UserInterfaceController;

extern crate termcolor;
use self::termcolor::{BufferWriter, ColorChoice};

pub struct GameStateController {}

impl Default for GameStateController {
    fn default() -> Self {
        Self::new()
    }
}

impl GameStateController {
    pub fn new() -> GameStateController {
        GameStateController {}
    }

    pub fn run(&self) -> Result<(), String> {
        let user_interface_controller =
            UserInterfaceController::new(BufferWriter::stdout(ColorChoice::Always));

        user_interface_controller.write_announcement("Welcome to 'Fire in the Lake'")?;

        user_interface_controller.write_announcement("Full scenario setup")?;

        user_interface_controller.write_section("Faction stats")?;

        let (_board, collection_of_instructions) = SetupController::new().setup_full()?;

        for instruction in collection_of_instructions[0].iter() {
            user_interface_controller.write_instruction(instruction.as_str())?;
        }

        user_interface_controller.write_section("Out of Play")?;

        for instruction in collection_of_instructions[1].iter() {
            user_interface_controller.write_instruction(instruction.as_str())?;
        }

        user_interface_controller.write_section("Forces in Spaces")?;

        for instruction in collection_of_instructions[2].iter() {
            user_interface_controller.write_instruction(instruction.as_str())?;
        }

        Ok(())
    }
}
