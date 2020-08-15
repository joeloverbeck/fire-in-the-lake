use std::io::Write;

extern crate termcolor;
use self::termcolor::{Buffer, Color, ColorSpec, WriteColor};

pub fn reset_console_output_to_normal(buffer: &mut Buffer) -> Result<&mut Buffer, String> {
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

    Ok(buffer)
}
