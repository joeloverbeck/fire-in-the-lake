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

pub fn write_output_tag_for_faction_stat<'a>(
    text: &str,
    buffer: &'a mut Buffer,
) -> Result<&'a mut Buffer, String> {
    if text == "[Aid]" {
        write_output_for_case(" Aid ", Color::Green, buffer)?;
    } else if text == "[TotalEcon]" {
        write_output_for_case(" Total Econ ", Color::Yellow, buffer)?;
    } else if text == "[Patronage]" {
        write_output_for_case(" Patronage ", Color::Yellow, buffer)?;
    } else if text == "[NvaResources]" {
        write_output_for_case(" NVA Resources ", Color::Red, buffer)?;
    } else if text == "[VcResources]" {
        write_output_for_case(" VC Resources ", Color::Blue, buffer)?;
    } else if text == "[ArvnResources]" {
        write_output_for_case(" ARVN Resources ", Color::Yellow, buffer)?;
    } else if text == "[SupportPlusAvailable]" {
        write_output_for_case(" Support + Available ", Color::Green, buffer)?;
    } else if text == "[CoinPlusPatronage]" {
        write_output_for_case(" COIN + Patronage ", Color::Yellow, buffer)?;
    } else if text == "[OppositionPlusBases]" {
        write_output_for_case(" Opposition + Bases ", Color::Blue, buffer)?;
    } else if text == "[NvaPlusBases]" {
        write_output_for_case(" NVA + Bases ", Color::Red, buffer)?;
    } else if text == "[TheTrail]" {
        write_output_for_case(" The Trail ", Color::Red, buffer)?;
    }

    Ok(buffer)
}
