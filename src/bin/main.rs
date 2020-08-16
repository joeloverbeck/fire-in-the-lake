extern crate fire_in_the_lake;
use fire_in_the_lake::game_state::controllers::game_state_controller::GameStateController;
/*
use fire_in_the_lake::board::controllers::setup_controller::SetupController;
use fire_in_the_lake::game_definitions::factions::Factions;

use fire_in_the_lake::players::controllers::players_controller::PlayersController;
use fire_in_the_lake::user_interface::controllers::user_interface_controller::UserInterfaceController;

extern crate termcolor;
use self::termcolor::{BufferWriter, ColorChoice};
*/

fn main() {
    if let Err(error) = GameStateController::new().run() {
        panic!("The game crashed for the following error: {:?}", error);
    }

    /*

    let mut players_controller = PlayersController::new();

    let setup_result = SetupController::new().setup_full();

    let user_interface_controller =
        UserInterfaceController::new(BufferWriter::stdout(ColorChoice::Always));

    if let Err(error) = setup_result {
        panic!("Couldn't setup the full scenario! Error: {:?}", error);
    } else if let Ok((board, _)) = setup_result {
        let mut possible_actions: Vec<String> = Vec::new();
        possible_actions.push("operation".to_string());
        possible_actions.push("event".to_string());
        possible_actions.push("pass".to_string());

        let decide_result = players_controller.decide(
            &Factions::US,
            16,
            84,
            possible_actions,
            &board,
            &user_interface_controller,
        );

        if let Err(error) = decide_result {
            panic!("{:?}", error);
        }

        println!("{:?}", decide_result.unwrap());
    } else {
        panic!("Not contemplated.");
    }

    */
}
