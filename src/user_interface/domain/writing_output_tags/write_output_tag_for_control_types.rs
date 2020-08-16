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
            .set_fg(Some(Color::Black))
            .set_bg(Some(background_color)),
    ) {
        return Err(error.to_string());
    }

    if let Err(error) = write!(buffer, "{}", faction_text) {
        return Err(error.to_string());
    }

    Ok(())
}

pub fn write_output_tag_for_control_types<'a>(
    text: &str,
    buffer: &'a mut Buffer,
) -> Result<&'a mut Buffer, String> {
    if text == "[CoinControl]" {
        write_output_for_case(" COIN Control ", Color::Yellow, buffer)?;
    } else if text == "[NvaControl]" {
        write_output_for_case(" NVA Control ", Color::Red, buffer)?;
    }

    Ok(buffer)
}
