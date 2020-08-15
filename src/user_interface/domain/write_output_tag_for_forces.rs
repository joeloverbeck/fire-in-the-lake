use std::io::Write;

extern crate termcolor;
use self::termcolor::{Buffer, Color, ColorSpec, WriteColor};

fn write_output_for_case(
    text: &str,
    background_color: Color,
    buffer: &mut Buffer,
) -> Result<(), String> {
    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Magenta))
            .set_bg(Some(background_color)),
    ) {
        return Err(error.to_string());
    }

    if let Err(error) = write!(buffer, "{}", text) {
        return Err(error.to_string());
    }

    Ok(())
}

pub fn write_output_tag_for_forces<'a>(
    text: &str,
    buffer: &'a mut Buffer,
) -> Result<&'a mut Buffer, String> {
    if text == "[UsBase]" {
        write_output_for_case(" US BASES ", Color::Green, buffer)?;
    } else if text == "[ArvnBase]" {
        write_output_for_case(" ARVN BASES ", Color::Yellow, buffer)?;
    } else if text == "[VcBase]" {
        write_output_for_case(" VC BASES ", Color::Blue, buffer)?;
    } else if text == "[TunneledVcBase]" {
        write_output_for_case(" TUNNELED VC BASES ", Color::Blue, buffer)?;
    } else if text == "[NvaBase]" {
        write_output_for_case(" NVA BASES ", Color::Red, buffer)?;
    } else if text == "[TunneledNvaBase]" {
        write_output_for_case(" TUNNELED NVA BASES ", Color::Red, buffer)?;
    } else if text == "[UsTroop]" {
        write_output_for_case(" US TROOPS ", Color::Green, buffer)?;
    } else if text == "[UndergroundUsIrregular]" {
        write_output_for_case(" UNDERGROUND US IRREGULARS ", Color::Green, buffer)?;
    } else if text == "[ActiveUsIrregular]" {
        write_output_for_case(" ACTIVE US IRREGULARS ", Color::Green, buffer)?;
    } else if text == "[ArvnTroop]" {
        write_output_for_case(" ARVN TROOPS ", Color::Yellow, buffer)?;
    } else if text == "[UndergroundArvnRanger]" {
        write_output_for_case(" UNDERGROUND ARVN RANGERS ", Color::Yellow, buffer)?;
    } else if text == "[ActiveArvnRanger]" {
        write_output_for_case(" ACTIVE ARVN RANGERS ", Color::Yellow, buffer)?;
    }

    Ok(buffer)
}
