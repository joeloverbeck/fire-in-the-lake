use board::controllers::setup_controller::SetupController;
use board::domain::board::Board;
use cards::controllers::cards_controller::CardsController;
use game_definitions::factions::Factions;
use players::controllers::players_controller::PlayersController;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use user_interface::controllers::user_interface_controller::UserInterfaceController;

extern crate termcolor;
use self::termcolor::{BufferWriter, ColorChoice};

pub struct GameStateController {
    board: Option<Board>,
}

impl Default for GameStateController {
    fn default() -> Self {
        Self::new()
    }
}

impl GameStateController {
    pub fn new() -> GameStateController {
        GameStateController { board: None }
    }

    fn delegate_setting_up_full_scenario(
        &mut self,
        user_interface_controller: &UserInterfaceController,
    ) -> Result<(), String> {
        user_interface_controller.write_announcement("Full scenario setup")?;

        user_interface_controller.write_section("Faction stats")?;

        let (board, collection_of_instructions) = SetupController::new().setup_full()?;

        self.board = Some(board);

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

    pub fn run(&mut self) -> Result<(), String> {
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
        let active_card_name = cards_controller.get_active_card_name()?;
        let faction_order =
            cards_controller.get_faction_order(active_card_in_text.parse::<u8>().unwrap())?;

        let mut players_controller = PlayersController::new();
        let mut sequence_of_play_controller = SequenceOfPlayController::new();

        let turn = 1;

        user_interface_controller.write_announcement(
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

        let current_eligible_faction = sequence_of_play_controller.get_current_elegible_faction();
        let possible_actions =
            sequence_of_play_controller.get_possible_actions_for_current_elegible();

        // Now the most complicated part. Depending on who is the current elegible, we must create a new
        // section and then delegate the decision of what to do to that faction's player (Human/AI).
        if let Some(faction) = current_eligible_faction {
            // Write section.
            user_interface_controller.write_section(format!("{} action", faction).as_str())?;

            let decision = players_controller.decide(
                faction,
                cards_controller.get_active_card()?,
                cards_controller.get_preview_card()?,
                possible_actions.unwrap(),
                self.board.as_ref().unwrap(),
                &user_interface_controller,
            )?;

            // The decision should have contained all the decisions.
            user_interface_controller.write_instructions_for_decision(&decision, faction)?;
        } else {
            panic!(
                "Not handled for faction: {:?}",
                current_eligible_faction.unwrap()
            );
        }

        Ok(())
    }
}
