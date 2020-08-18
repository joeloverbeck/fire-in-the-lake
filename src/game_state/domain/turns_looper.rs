use board::domain::board::Board;
use cards::controllers::cards_controller::CardsController;
use game_state::domain::player_action_phases_looper::PlayerActionPhasesLooper;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

pub struct TurnsLooper {
    player_action_phases_looper: PlayerActionPhasesLooper,
    sequence_of_play_controller: SequenceOfPlayController,
    turn: u16,
}

impl Default for TurnsLooper {
    fn default() -> Self {
        Self::new()
    }
}

impl TurnsLooper {
    pub fn new() -> TurnsLooper {
        TurnsLooper {
            sequence_of_play_controller: SequenceOfPlayController::new(),
            player_action_phases_looper: PlayerActionPhasesLooper::new(),
            turn: 1,
        }
    }

    pub fn run(
        &mut self,
        cards_controller: &mut CardsController,
        board: &mut Board,
        display_controller: &DisplayController,
        keyboard_input_controller: &KeyboardInputController,
    ) -> Result<(), String> {
        loop {
            let active_card_name = cards_controller.get_active_card_name()?;
            let faction_order =
                cards_controller.get_faction_order(cards_controller.get_active_card()?)?;

            display_controller.write_announcement(
                format!(
                    "Turn {}: '{}'  {} {} {} {}  ",
                    self.turn,
                    active_card_name,
                    faction_order[0],
                    faction_order[1],
                    faction_order[2],
                    faction_order[3]
                )
                .as_str(),
            )?;

            self.sequence_of_play_controller.register_faction_order(
                cards_controller.get_faction_order(cards_controller.get_active_card()?)?,
            )?;

            self.player_action_phases_looper.run(
                board,
                cards_controller,
                &mut self.sequence_of_play_controller,
                &keyboard_input_controller,
                &display_controller,
            )?;

            self.turn += 1;

            // Obviously should change this at some point.
            if self.turn == 10 {
                break;
            }
        }

        Ok(())
    }
}
