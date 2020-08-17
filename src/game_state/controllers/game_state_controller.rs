use board::controllers::setup_controller::SetupController;
use board::domain::board::Board;
use cards::controllers::cards_controller::CardsController;
use game_definitions::factions::Factions;
use game_state::domain::player_action_phases_looper::PlayerActionPhasesLooper;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

extern crate termcolor;
use self::termcolor::{BufferWriter, ColorChoice};

pub struct GameStateController {
    board: Option<Board>,
    player_action_phases_looper: PlayerActionPhasesLooper,
}

impl Default for GameStateController {
    fn default() -> Self {
        Self::new()
    }
}

impl GameStateController {
    pub fn new() -> GameStateController {
        GameStateController {
            board: None,
            player_action_phases_looper: PlayerActionPhasesLooper::new(),
        }
    }

    fn delegate_setting_up_full_scenario(
        &mut self,
        display_controller: &DisplayController,
    ) -> Result<(), String> {
        display_controller.write_announcement("Full scenario setup")?;

        display_controller.write_section("Faction stats")?;

        let (board, collection_of_instructions) = SetupController::new().setup_full()?;

        self.board = Some(board);

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

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        let display_controller = DisplayController::new(BufferWriter::stdout(ColorChoice::Always));
        let keyboard_input_controller =
            KeyboardInputController::new(BufferWriter::stdout(ColorChoice::Always));

        display_controller.write_announcement("Welcome to 'Fire in the Lake'")?;

        self.delegate_setting_up_full_scenario(&display_controller)?;

        display_controller.write_announcement("Game start")?;

        display_controller.write_instruction(
            "Reveal the top card of the draw deck and place it onto a played cards pile.",
        )?;

        let active_card_in_text = keyboard_input_controller
            .request_player_input("What is the number of the card on the played card stack?: ")?;

        let mut cards_controller = CardsController::new();

        cards_controller.set_active_card(active_card_in_text.parse::<u8>().unwrap())?;

        display_controller.write_instruction("Reveal the next card on top of the draw deck.")?;

        let preview_card_in_text = keyboard_input_controller
            .request_player_input("What is the number of the card on top of the draw deck?: ")?;

        cards_controller.set_preview_card(preview_card_in_text.parse::<u8>().unwrap())?;

        // We have all we need to start the game.
        let active_card_name = cards_controller.get_active_card_name()?;
        let faction_order =
            cards_controller.get_faction_order(active_card_in_text.parse::<u8>().unwrap())?;

        let mut sequence_of_play_controller = SequenceOfPlayController::new();

        let mut turn = 1;

        loop {
            display_controller.write_announcement(
                format!(
                    "Turn {}: '{}'  {} {} {} {}  ",
                    turn,
                    active_card_name,
                    faction_order[0],
                    faction_order[1],
                    faction_order[2],
                    faction_order[3]
                )
                .as_str(),
            )?;

            sequence_of_play_controller.register_faction_order(
                *cards_controller.get_faction_order(cards_controller.get_active_card()?)?,
            )?;

            self.player_action_phases_looper.run(
                self.board.as_mut().unwrap(),
                &cards_controller,
                &mut sequence_of_play_controller,
                &keyboard_input_controller,
                &display_controller,
            )?;

            turn += 1;

            if turn == 10 {
                break;
            }
        }

        Ok(())
    }
}
