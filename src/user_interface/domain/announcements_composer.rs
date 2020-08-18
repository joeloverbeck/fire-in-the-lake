use std::io::Write;
use user_interface::domain::isolate_game_definitions_in_text::isolate_game_definitions_in_text;
use user_interface::domain::reset_console_output_to_normal::reset_console_output_to_normal;
use user_interface::domain::writing_output_tags::write_output_tag_for_faction::write_output_tag_for_faction;

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

    fn set_console_output_to_standard_for_announcement(
        &self,
        buffer: &mut Buffer,
    ) -> Result<(), String> {
        if let Err(error) = buffer.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Black))
                .set_bg(Some(Color::White)),
        ) {
            return Err(error.to_string());
        }

        Ok(())
    }

    pub fn compose(&self, text: &str, buffer: &'a mut Buffer) -> Result<&'a mut Buffer, String> {
        // Bookend the announcement with empty lines.
        reset_console_output_to_normal(buffer)?;
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

        self.set_console_output_to_standard_for_announcement(buffer)?;

        // As in the case of the instructions, I'll have to reconstruct the text
        // in case that we need to paint the contained game definitions in a certain way
        let separated_text = isolate_game_definitions_in_text(text)?;

        for entry in separated_text {
            // Can be a faction tag
            if entry == "[VC]" || entry == "[ARVN]" || entry == "[US]" || entry == "[NVA]" {
                write_output_tag_for_faction(entry.as_str(), buffer)?;
            } else {
                // Gotta change settings to the standard for an announcement, just in case.
                self.set_console_output_to_standard_for_announcement(buffer)?;

                if let Err(error) = write!(buffer, "{}", entry.to_uppercase()) {
                    return Err(error.to_string());
                }
            }
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
