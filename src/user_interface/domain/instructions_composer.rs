use std::io::Write;
use text_manipulation::replace_extraneous_characters_from_text::replace_extraneous_characters_from_text;
use user_interface::domain::does_text_refer_to_a_faction_stat::does_text_refer_to_a_faction_stat;
use user_interface::domain::does_text_refer_to_forces::does_text_refer_to_forces;
use user_interface::domain::does_text_refer_to_space::does_text_refer_to_a_space;
use user_interface::domain::does_text_unit_have_special_meaning::does_text_unit_have_special_meaning;
use user_interface::domain::reset_console_output_to_normal::reset_console_output_to_normal;
use user_interface::domain::write_output_tag_for_faction::write_output_tag_for_faction;
use user_interface::domain::write_output_tag_for_faction_stat::write_output_tag_for_faction_stat;
use user_interface::domain::write_output_tag_for_forces::write_output_tag_for_forces;
use user_interface::domain::write_regular_text::write_regular_text;

extern crate termcolor;
use self::termcolor::{Buffer, Color, ColorSpec, WriteColor};

pub struct InstructionsComposer {
    instruction_tag: String,
}

impl Default for InstructionsComposer {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> InstructionsComposer {
    pub fn new() -> InstructionsComposer {
        InstructionsComposer {
            instruction_tag: "  ==>".to_string(),
        }
    }

    pub fn compose(&self, text: &str, buffer: &'a mut Buffer) -> Result<&'a mut Buffer, String> {
        // Write the instruction "tag".

        if let Err(error) = buffer.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Cyan))
                .set_bg(Some(Color::Black)),
        ) {
            return Err(error.to_string());
        }

        if let Err(error) = write!(buffer, "{}", self.instruction_tag) {
            return Err(error.to_string());
        }

        // Note: there might be tags in the text. We gotta find them and change
        let split_text_iter = text.split_whitespace();

        let mut plain_text: String = "".to_string();

        let mut separated_text: Vec<String> = Vec::new();

        for entry in split_text_iter {
            if does_text_unit_have_special_meaning(entry) {
                // if there's something in plain text, push it into the vec and clean the current
                // plain text
                if !plain_text.is_empty() {
                    separated_text.push(plain_text + " ");
                    plain_text = "".to_string();
                }

                separated_text.push(entry.to_string());
            } else {
                plain_text = plain_text + " " + entry;
            }
        }

        // If anything remains in plain_text, push it to the vec.
        if !plain_text.is_empty() {
            separated_text.push(plain_text);
        }

        for entry in separated_text.iter() {
            if entry == "[VC]" || entry == "[ARVN]" || entry == "[US]" || entry == "[NVA]" {
                write_output_tag_for_faction(entry, buffer)?;
            } else if does_text_refer_to_a_space(entry) {
                let filtered_text = replace_extraneous_characters_from_text(entry);

                if let Err(error) = buffer.set_color(
                    ColorSpec::new()
                        .set_fg(Some(Color::Red))
                        .set_bg(Some(Color::White)),
                ) {
                    return Err(error.to_string());
                }

                if let Err(error) = write!(buffer, " {} ", filtered_text) {
                    return Err(error.to_string());
                }
            } else if does_text_refer_to_a_faction_stat(entry) {
                write_output_tag_for_faction_stat(entry, buffer)?;
            } else if does_text_refer_to_forces(entry) {
                write_output_tag_for_forces(entry, buffer)?;
            } else if replace_extraneous_characters_from_text(entry)
                .parse::<u8>()
                .is_ok()
            {
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
                    replace_extraneous_characters_from_text(entry)
                ) {
                    return Err(error.to_string());
                }
            } else {
                write_regular_text(entry, buffer)?;
            }
        }

        reset_console_output_to_normal(buffer)?;

        Ok(buffer)
    }
}
