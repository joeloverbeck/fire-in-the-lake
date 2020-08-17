use std::io::Write;
use user_interface::domain::reset_console_output_to_normal::reset_console_output_to_normal;

extern crate termcolor;
use self::termcolor::{Buffer, Color, ColorSpec, WriteColor};

pub struct SectionWriter {
    section_tag_start: String,
    section_tag_end: String,
}

impl Default for SectionWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> SectionWriter {
    pub fn new() -> SectionWriter {
        SectionWriter {
            section_tag_start: " ///".to_string(),
            section_tag_end: "///".to_string(),
        }
    }

    pub fn write(&self, text: &str, buffer: &'a mut Buffer) -> Result<&'a mut Buffer, String> {
        reset_console_output_to_normal(buffer)?;

        // Write the instruction "tag".
        if let Err(error) = buffer.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Red))
                .set_bg(Some(Color::Black)),
        ) {
            return Err(error.to_string());
        }

        if let Err(error) = write!(buffer, "{}", self.section_tag_start) {
            return Err(error.to_string());
        }

        // Write the actual text regarding the section
        if let Err(error) = buffer.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::White))
                .set_bg(Some(Color::Black)),
        ) {
            return Err(error.to_string());
        }

        if let Err(error) = write!(buffer, "{}", text.to_uppercase()) {
            return Err(error.to_string());
        }

        if let Err(error) = buffer.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Red))
                .set_bg(Some(Color::Black)),
        ) {
            return Err(error.to_string());
        }

        if let Err(error) = write!(buffer, "{}", self.section_tag_end) {
            return Err(error.to_string());
        }

        reset_console_output_to_normal(buffer)?;
        reset_console_output_to_normal(buffer)?;

        Ok(buffer)
    }
}
