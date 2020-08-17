use std::io::{stdout, Write};

extern crate termcolor;
use self::termcolor::{BufferWriter, Color, ColorSpec, WriteColor};
pub struct PlayerInputRequester {}

impl Default for PlayerInputRequester {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> PlayerInputRequester {
    pub fn new() -> PlayerInputRequester {
        PlayerInputRequester {}
    }

    pub fn request(&self, text: &str, buffer_writer: &BufferWriter) -> Result<String, String> {
        let mut buffer = buffer_writer.buffer();

        if let Err(error) = buffer.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Green))
                .set_bg(Some(Color::Black)),
        ) {
            return Err(error.to_string());
        }

        if let Err(error) = write!(buffer, "  >> {}", text) {
            return Err(error.to_string());
        }

        if let Err(error) = buffer.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::White))
                .set_bg(Some(Color::Black)),
        ) {
            return Err(error.to_string());
        }

        if let Err(error) = write!(buffer, "") {
            return Err(error.to_string());
        }

        let buffer_writer_result = buffer_writer.print(&buffer);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        stdout().flush().unwrap();

        let mut input = String::new();

        if let Err(error) = std::io::stdin().read_line(&mut input) {
            return Err(error.to_string());
        }

        if let Err(error) = buffer.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::White))
                .set_bg(Some(Color::Black)),
        ) {
            return Err(error.to_string());
        }

        if let Err(error) = writeln!(buffer) {
            return Err(error.to_string());
        }

        Ok(input.trim_end().to_string())
    }
}
