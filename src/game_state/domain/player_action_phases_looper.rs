use board::domain::board::Board;
use cards::controllers::cards_controller::CardsController;
use flags::controllers::flags_controller::FlagsController;
use persistence::controllers::memory_persistence_controller::MemoryPersistenceController;
use players::controllers::players_controller::PlayersController;
use sequence_of_play::controllers::sequence_of_play_controller::SequenceOfPlayController;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::controllers::keyboard_input_controller::KeyboardInputController;

pub struct PlayerActionPhasesLooper {
    players_controller: PlayersController,
    flags_controller: FlagsController,
    memory_persistence_controller: MemoryPersistenceController,
}

impl Default for PlayerActionPhasesLooper {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerActionPhasesLooper {
    pub fn new() -> PlayerActionPhasesLooper {
        PlayerActionPhasesLooper {
            players_controller: PlayersController::new(),
            flags_controller: FlagsController::new(),
            memory_persistence_controller: MemoryPersistenceController::new(),
        }
    }

    pub fn run(
        &mut self,
        board: &mut Board,
        cards_controller: &CardsController,
        sequence_of_play_controller: &mut SequenceOfPlayController,
        keyboard_input_controller: &KeyboardInputController,
        display_controller: &DisplayController,
    ) -> Result<(), String> {
        loop {
            // Now the most complicated part. Depending on who is the current elegible, we must create a new
            // section and then delegate the decision of what to do to that faction's player (Human/AI).
            if sequence_of_play_controller.is_there_a_next_eligible_faction()? {
                let faction = sequence_of_play_controller
                    .get_current_elegible_faction()
                    .as_ref()
                    .unwrap();

                // Write section.
                display_controller.write_section(format!(" {} Action Phase ", faction).as_str())?;

                let decision = self.players_controller.decide(
                    faction,
                    cards_controller.get_active_card()?,
                    cards_controller.get_preview_card()?,
                    sequence_of_play_controller
                        .get_possible_actions_for_current_elegible()
                        .unwrap(),
                    board,
                    &keyboard_input_controller,
                    &display_controller,
                )?;

                // The decision should have contained all the decisions.
                display_controller.write_instructions_for_decision(&decision, faction)?;

                // Delegate persisting the changes.
                self.memory_persistence_controller.persist_decision(
                    &decision,
                    board,
                    *cards_controller.get_faction_order(cards_controller.get_active_card()?)?,
                    sequence_of_play_controller,
                    &mut self.flags_controller,
                )?;
            } else {
                break;
            }
        }

        Ok(())
    }
}
