use board::controllers::setup_controller::SetupController;
use cards::controllers::cards_controller::CardsController;
use game_definitions::factions::Factions;
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

    fn delegate_setting_up_full_scenario(
        &self,
        user_interface_controller: &UserInterfaceController,
    ) -> Result<(), String> {
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

        user_interface_controller.write_section("Control")?;

        for instruction in collection_of_instructions[3].iter() {
            user_interface_controller.write_instruction(instruction.as_str())?;
        }

        user_interface_controller.write_section("Support")?;

        for instruction in collection_of_instructions[4].iter() {
            user_interface_controller.write_instruction(instruction.as_str())?;
        }

        user_interface_controller.write_section("Faction elegibility")?;

        user_interface_controller.write_instruction(
            format!(
                "Set the cylinders of all factions ( {} {} {} {} ) into the Eligible box.",
                Factions::US,
                Factions::ARVN,
                Factions::NVA,
                Factions::VC
            )
            .as_str(),
        )?;

        Ok(())
    }

    pub fn run(&self) -> Result<(), String> {
        let user_interface_controller =
            UserInterfaceController::new(BufferWriter::stdout(ColorChoice::Always));

        user_interface_controller.write_announcement("Welcome to 'Fire in the Lake'")?;

        self.delegate_setting_up_full_scenario(&user_interface_controller)?;

        user_interface_controller.write_announcement("Game start")?;

        user_interface_controller.write_instruction(
            "Reveal the top card of the draw deck and place it onto a played cards pile.",
        )?;

        let active_card_in_text = user_interface_controller
            .request_player_input("What is the number of the card on the played card stack?: ")?;

        let mut cards_controller = CardsController::new();

        cards_controller.set_active_card(active_card_in_text.parse::<u8>().unwrap())?;

        user_interface_controller
            .write_instruction("Reveal the next card on top of the draw deck.")?;

        let preview_card_in_text = user_interface_controller
            .request_player_input("What is the number of the card on top of the draw deck?: ")?;

        cards_controller.set_preview_card(preview_card_in_text.parse::<u8>().unwrap())?;

        // We have all we need to start the game.

        Ok(())
    }
}
