extern crate fire_in_the_lake;
use fire_in_the_lake::game_state::controllers::game_state_controller::GameStateController;

fn main() {
    if let Err(error) = GameStateController::new().run() {
        panic!("The game crashed for the following error: {:?}", error);
    }
}
