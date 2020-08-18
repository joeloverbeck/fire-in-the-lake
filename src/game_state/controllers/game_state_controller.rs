use cards::controllers::cards_controller::CardsController;
use game_state::domain::run_pre_game::run_pre_game;
use game_state::domain::turns_looper::TurnsLooper;
use scenario_building::controllers::scenario_building_controller::ScenarioBuildingController;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::display_controller_trait::DisplayControllerTrait;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

extern crate termcolor;
use self::termcolor::{BufferWriter, ColorChoice};

pub struct GameStateController {
    scenario_building_controller: ScenarioBuildingController,
    turns_looper: TurnsLooper,
}

impl Default for GameStateController {
    fn default() -> Self {
        Self::new()
    }
}

impl GameStateController {
    pub fn new() -> GameStateController {
        GameStateController {
            scenario_building_controller: ScenarioBuildingController::new(),
            turns_looper: TurnsLooper::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let display_controller = DisplayController::new(BufferWriter::stdout(ColorChoice::Always));
        let keyboard_input_controller =
            KeyboardInputController::new(BufferWriter::stdout(ColorChoice::Always));

        display_controller.write_announcement("Welcome to 'Fire in the Lake'")?;

        let mut board = self
            .scenario_building_controller
            .build_full_scenario(&display_controller)?;

        let mut cards_controller = CardsController::new();

        run_pre_game(
            &mut cards_controller,
            &display_controller,
            &keyboard_input_controller,
        )?;

        // We have all we need to start the game.
        self.turns_looper.run(
            &mut cards_controller,
            &mut board,
            &display_controller,
            &keyboard_input_controller,
        )?;

        Ok(())
    }
}
