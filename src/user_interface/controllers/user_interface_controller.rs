use user_interface::domain::announcements_composer::AnnouncementsComposer;

extern crate termcolor;
use self::termcolor::BufferWriter;

pub struct UserInterfaceController {
    buffer_writer: BufferWriter,
    announcements_composer: AnnouncementsComposer,
}

impl UserInterfaceController {
    pub fn new(buffer_writer: BufferWriter) -> UserInterfaceController {
        UserInterfaceController {
            buffer_writer,
            announcements_composer: AnnouncementsComposer::new(),
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
}
