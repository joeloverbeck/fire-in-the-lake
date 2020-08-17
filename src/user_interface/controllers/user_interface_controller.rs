use game_definitions::factions::Factions;
use players::domain::decision::Decision;
use user_interface::domain::alert_writer::AlertWriter;
use user_interface::domain::announcements_composer::AnnouncementsComposer;
use user_interface::domain::input_violation_types::InputViolationTypes;
use user_interface::domain::instructions_composer::InstructionsComposer;
use user_interface::domain::player_input_requester::PlayerInputRequester;
use user_interface::domain::produce_instructions_for_mutations::produce_instructions_for_mutations;
use user_interface::domain::section_writer::SectionWriter;

extern crate termcolor;
use self::termcolor::BufferWriter;

pub struct UserInterfaceController {
    buffer_writer: BufferWriter,
    announcements_composer: AnnouncementsComposer,
    section_writer: SectionWriter,
    instructions_composer: InstructionsComposer,
    player_input_requester: PlayerInputRequester,
    alert_writer: AlertWriter,
}

impl UserInterfaceController {
    pub fn new(buffer_writer: BufferWriter) -> UserInterfaceController {
        UserInterfaceController {
            buffer_writer,
            announcements_composer: AnnouncementsComposer::new(),
            section_writer: SectionWriter::new(),
            instructions_composer: InstructionsComposer::new(),
            player_input_requester: PlayerInputRequester::new(),
            alert_writer: AlertWriter::new(),
        }
    }

    pub fn write_announcement(&self, text: &str) -> Result<(), String> {
        let buffer = &mut self.buffer_writer.buffer();

        let buffer_writer_result = self
            .buffer_writer
            .print(self.announcements_composer.compose(text, buffer)?);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        Ok(())
    }

    pub fn write_instructions_for_decision(
        &self,
        decision: &Decision,
        faction: &Factions,
    ) -> Result<(), String> {
        let instructions = produce_instructions_for_mutations(decision, faction)?;

        for instruction in instructions.iter() {
            self.write_instruction(instruction)?;
        }

        Ok(())
    }

    pub fn write_instruction(&self, text: &str) -> Result<(), String> {
        let buffer = &mut self.buffer_writer.buffer();

        let buffer_writer_result = self
            .buffer_writer
            .print(self.instructions_composer.compose(text, buffer)?);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        Ok(())
    }

    pub fn write_section(&self, text: &str) -> Result<(), String> {
        let buffer = &mut self.buffer_writer.buffer();

        let buffer_writer_result = self
            .buffer_writer
            .print(self.section_writer.write(text, buffer)?);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        Ok(())
    }

    pub fn write_alert(&self, text: &str) -> Result<(), String> {
        let buffer = &mut self.buffer_writer.buffer();

        let buffer_writer_result = self
            .buffer_writer
            .print(self.alert_writer.write(text, buffer)?);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        Ok(())
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
                    self.write_alert(violation_message.as_str())?;

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
