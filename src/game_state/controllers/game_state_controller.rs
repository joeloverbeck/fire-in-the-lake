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

        user_interface_controller.write_announcement("Full game scenario setup")?;

        user_interface_controller
            .write_instruction("Place a {VC} troop in {SAIGON}, you dangy!")?;

        Ok(())
    }
}
