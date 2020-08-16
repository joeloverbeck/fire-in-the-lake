use std::io::Write;
use text_manipulation::replace_extraneous_characters_from_text::replace_extraneous_characters_from_text;

extern crate termcolor;
use self::termcolor::{Buffer, Color, ColorSpec, WriteColor};

pub fn write_output_tag_for_numbers<'a>(
    text: &str,
    buffer: &'a mut Buffer,
) -> Result<&'a mut Buffer, String> {
    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Black))
            .set_bg(Some(Color::White)),
    ) {
        return Err(error.to_string());
    }

    if let Err(error) = write!(
        buffer,
        " {} ",
        replace_extraneous_characters_from_text(text)
    ) {
        return Err(error.to_string());
    }

    Ok(buffer)
}
