use std::io::Write;
use user_interface::domain::reset_console_output_to_normal::reset_console_output_to_normal;

extern crate termcolor;
use self::termcolor::{Buffer, Color, ColorSpec, WriteColor};

pub struct AlertWriter {}

impl Default for AlertWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> AlertWriter {
    pub fn new() -> AlertWriter {
        AlertWriter {}
    }

    pub fn write(&self, text: &str, buffer: &'a mut Buffer) -> Result<&'a mut Buffer, String> {
        reset_console_output_to_normal(buffer)?;

        // Write the actual text regarding the alert
        if let Err(error) = buffer.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Black))
                .set_bg(Some(Color::Red)),
        ) {
            return Err(error.to_string());
        }

        if let Err(error) = write!(buffer, " --{}-- ", text.to_uppercase()) {
            return Err(error.to_string());
        }

        reset_console_output_to_normal(buffer)?;
        reset_console_output_to_normal(buffer)?;

        Ok(buffer)
    }
}
