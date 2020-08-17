extern crate termcolor;
use self::termcolor::BufferWriter;
use user_interface::controllers::display_controller::DisplayController;
use user_interface::domain::input_violation_types::InputViolationTypes;
use user_interface::domain::player_input_requester::PlayerInputRequester;

pub struct KeyboardInputController {
    buffer_writer: BufferWriter,
    player_input_requester: PlayerInputRequester,
}

impl KeyboardInputController {
    pub fn new(buffer_writer: BufferWriter) -> KeyboardInputController {
        KeyboardInputController {
            buffer_writer,
            player_input_requester: PlayerInputRequester::new(),
        }
    }

    pub fn request_player_input(&self, text: &str) -> Result<String, String> {
        let player_input = self
            .player_input_requester
            .request(text, &self.buffer_writer)?;

        Ok(player_input)
    }

    pub fn request_player_input_configurable(
        &self,
        text: &str,
        out_text: Option<&str>,
    ) -> Result<(String, bool), String> {
        // Check if the player has inputted the "out_text". In that case we return bool signalling that the player
        // has said that it wants whatever process its being asked about to stop.
        let player_input = self
            .player_input_requester
            .request(text, &self.buffer_writer)?;

        if let Some(out_text_content) = out_text {
            let wants_out = player_input.to_lowercase() == out_text_content.to_lowercase();

            if wants_out {
                return Ok((player_input, wants_out));
            }
        }

        Ok((player_input, false))
    }

    pub fn request_numeric_player_input_configurable(
        &self,
        text: &str,
        out_text: Option<&str>,
        input_violations: Option<Vec<(u8, InputViolationTypes, String)>>,
        display_controller: &DisplayController,
    ) -> Result<(u8, bool), String> {
        // Check if the player has inputted the "out_text". In that case we return bool signalling that the player
        // has said that it wants whatever process its being asked about to stop.
        let player_input = self
            .player_input_requester
            .request(text, &self.buffer_writer)?;

        if let Some(out_text_content) = out_text {
            let wants_out = player_input.to_lowercase() == out_text_content.to_lowercase();

            if wants_out {
                return Ok((0, wants_out));
            }
        }

        let parsed_player_input: u8 = player_input.parse::<u8>().unwrap();

        if let Some(input_violations_contents) = input_violations {
            for input_violation in input_violations_contents {
                let (limit, input_violation_type, violation_message) = input_violation;

                if (input_violation_type == InputViolationTypes::Bigger
                    && parsed_player_input > limit)
                    || (input_violation_type == InputViolationTypes::Smaller
                        && parsed_player_input < limit)
                {
                    display_controller.write_alert(violation_message.as_str())?;

                    return Err(format!(
                        "Request player input failed with error: {:?}",
                        violation_message
                    ));
                }
            }
        }

        Ok((parsed_player_input, false))
    }
}
