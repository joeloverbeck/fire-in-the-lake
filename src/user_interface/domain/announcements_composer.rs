use std::io::Write;
use user_interface::domain::reset_console_output_to_normal::reset_console_output_to_normal;

extern crate termcolor;
use self::termcolor::{Buffer, Color, ColorSpec, WriteColor};

pub struct AnnouncementsComposer {
    bookend: String,
}

impl Default for AnnouncementsComposer {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> AnnouncementsComposer {
    pub fn new() -> AnnouncementsComposer {
        AnnouncementsComposer {
            bookend: " *** ".to_string(),
        }
    }

    pub fn compose(&self, text: &str, buffer: &'a mut Buffer) -> Result<&'a mut Buffer, String> {
        // Bookend the announcement with empty lines.
        reset_console_output_to_normal(buffer)?;

        if let Err(error) = buffer.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Red))
                .set_bg(Some(Color::White)),
        ) {
            return Err(error.to_string());
        }

        if let Err(error) = write!(buffer, "{}", self.bookend) {
            return Err(error.to_string());
        }

        if let Err(error) = buffer.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Black))
                .set_bg(Some(Color::White)),
        ) {
            return Err(error.to_string());
        }

        if let Err(error) = write!(buffer, "{}", text.to_uppercase()) {
            return Err(error.to_string());
        }

        if let Err(error) = buffer.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Red))
                .set_bg(Some(Color::White)),
        ) {
            return Err(error.to_string());
        }

        if let Err(error) = write!(buffer, "{}", self.bookend) {
            return Err(error.to_string());
        }

        // Doing it twice because it's nice.
        reset_console_output_to_normal(buffer)?;
        reset_console_output_to_normal(buffer)?;

        Ok(buffer)
    }
}
