use std::io::Write;

extern crate termcolor;
use self::termcolor::{Buffer, Color, ColorSpec, WriteColor};

fn write_output_for_case(
    faction_text: &str,
    background_color: Color,
    buffer: &mut Buffer,
) -> Result<(), String> {
    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_bg(Some(background_color)),
    ) {
        return Err(error.to_string());
    }

    if let Err(error) = write!(buffer, "{}", faction_text) {
        return Err(error.to_string());
    }

    Ok(())
}

pub fn write_output_tag_for_faction<'a>(
    text: &str,
    buffer: &'a mut Buffer,
) -> Result<&'a mut Buffer, String> {
    if text == "[VC]" {
        write_output_for_case(" VC ", Color::Blue, buffer)?;
    } else if text == "[ARVN]" {
        write_output_for_case(" ARVN ", Color::Yellow, buffer)?;
    } else if text == "[US]" {
        write_output_for_case(" US ", Color::Green, buffer)?;
    } else if text == "[NVA]" {
        write_output_for_case(" NVA ", Color::Red, buffer)?;
    }

    Ok(buffer)
}
