use board::domain::board::Board;
use players::domain::decision::Decision;
use players::domain::events::unshaded::produce_decision_for_unshaded_event_when_us_human::produce_decision_for_unshaded_event_when_us_human;
use user_interface::controllers::user_interface_controller::UserInterfaceController;

pub struct HumanUsPlayer {}

impl Default for HumanUsPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl HumanUsPlayer {
    pub fn new() -> HumanUsPlayer {
        HumanUsPlayer {}
    }

    pub fn decide(
        &self,
        active_card: u8,
        _preview_card: u8,
        possible_actions: Vec<String>,
        board: &Board,
        user_interface_controller: &UserInterfaceController,
    ) -> Result<Decision, String> {
        let mut possible_actions_text = "[".to_string();

        for entry in possible_actions {
            possible_actions_text += &entry;
        }

        possible_actions_text += &"]".to_string();

        let input = user_interface_controller.request_player_input(
            format!(
                "What action do you want to take? {}: ",
                possible_actions_text
            )
            .as_str(),
        )?;

        if input == "event" {
            // Many, many things can vary here. You first neet to take into account which card we are dealing with.
            // However, the cards controller cannot help us here, as it would be unreasonable
            // to codify any card event mechanics there. So there should be some general functions that deal with what
            // must be applied, or could even be chosen, regarding every event.
            Ok(produce_decision_for_unshaded_event_when_us_human(
                active_card,
                board,
                user_interface_controller,
            )?)
        } else {
            todo!()
        }
    }
}
