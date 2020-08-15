use user_interface::domain::announcements_composer::AnnouncementsComposer;
use user_interface::domain::instructions_composer::InstructionsComposer;
use user_interface::domain::player_input_requester::PlayerInputRequester;

extern crate termcolor;
use self::termcolor::BufferWriter;

pub struct UserInterfaceController {
    buffer_writer: BufferWriter,
    announcements_composer: AnnouncementsComposer,
    instructions_composer: InstructionsComposer,
    player_input_requester: PlayerInputRequester,
}

impl UserInterfaceController {
    pub fn new(buffer_writer: BufferWriter) -> UserInterfaceController {
        UserInterfaceController {
            buffer_writer,
            announcements_composer: AnnouncementsComposer::new(),
            instructions_composer: InstructionsComposer::new(),
            player_input_requester: PlayerInputRequester::new(),
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

    pub fn request_player_input(&self, text: &str) -> Result<String, String> {
        let player_input = self.player_input_requester.request(text)?;

        Ok(player_input)
    }
}
