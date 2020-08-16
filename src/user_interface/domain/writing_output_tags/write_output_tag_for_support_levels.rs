use std::io::Write;

extern crate termcolor;
use self::termcolor::{Buffer, Color, ColorSpec, WriteColor};

fn write_output_for_case(
    faction_text: &str,
    foreground_color: Color,
    buffer: &mut Buffer,
) -> Result<(), String> {
    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(foreground_color))
            .set_bg(Some(Color::Cyan)),
    ) {
        return Err(error.to_string());
    }

    if let Err(error) = write!(buffer, "{}", faction_text) {
        return Err(error.to_string());
    }

    Ok(())
}

pub fn write_output_tag_for_support_levels<'a>(
    text: &str,
    buffer: &'a mut Buffer,
) -> Result<&'a mut Buffer, String> {
    if text == "[ActiveSupport]" {
        write_output_for_case(" Active Support ", Color::Yellow, buffer)?;
    } else if text == "[PassiveSupport]" {
        write_output_for_case(" Passive Support ", Color::Yellow, buffer)?;
    } else if text == "[PassiveOpposition]" {
        write_output_for_case(" Passive Opposition ", Color::Red, buffer)?;
    } else if text == "[ActiveOpposition]" {
        write_output_for_case(" Active Opposition ", Color::Red, buffer)?;
    }

    Ok(buffer)
}
