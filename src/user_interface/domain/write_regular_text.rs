use std::io::Write;

extern crate termcolor;
use self::termcolor::{Buffer, Color, ColorSpec, WriteColor};

pub fn write_regular_text(text: &str, buffer: &mut Buffer) -> Result<(), String> {
    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_bg(Some(Color::Black)),
    ) {
        return Err(error.to_string());
    }

    if let Err(error) = write!(buffer, "{}", text) {
        return Err(error.to_string());
    }

    Ok(())
}
