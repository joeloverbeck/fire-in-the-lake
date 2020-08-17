use std::io::Write;
use text_manipulation::replace_extraneous_characters_from_text::replace_extraneous_characters_from_text;
use user_interface::domain::checking_game_definitions::does_text_refer_to_a_faction_stat::does_text_refer_to_a_faction_stat;
use user_interface::domain::checking_game_definitions::does_text_refer_to_control_types::does_text_refer_to_control_types;
use user_interface::domain::checking_game_definitions::does_text_refer_to_forces::does_text_refer_to_forces;
use user_interface::domain::checking_game_definitions::does_text_refer_to_space::does_text_refer_to_a_space;
use user_interface::domain::checking_game_definitions::does_text_refer_to_support_levels::does_text_refer_to_support_levels;
use user_interface::domain::isolate_game_definitions_in_text::isolate_game_definitions_in_text;
use user_interface::domain::reset_console_output_to_normal::reset_console_output_to_normal;
use user_interface::domain::write_regular_text::write_regular_text;
use user_interface::domain::writing_output_tags::write_output_tag_for_control_types::write_output_tag_for_control_types;
use user_interface::domain::writing_output_tags::write_output_tag_for_faction::write_output_tag_for_faction;
use user_interface::domain::writing_output_tags::write_output_tag_for_faction_stat::write_output_tag_for_faction_stat;
use user_interface::domain::writing_output_tags::write_output_tag_for_forces::write_output_tag_for_forces;
use user_interface::domain::writing_output_tags::write_output_tag_for_numbers::write_output_tag_for_numbers;
use user_interface::domain::writing_output_tags::write_output_tag_for_space::write_output_tag_for_space;
use user_interface::domain::writing_output_tags::write_output_tag_for_support_levels::write_output_tag_for_support_levels;

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
            instruction_tag: "  [<>]".to_string(),
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

        let separated_text = isolate_game_definitions_in_text(text)?;

        for entry in separated_text.iter() {
            if entry == "[VC]" || entry == "[ARVN]" || entry == "[US]" || entry == "[NVA]" {
                write_output_tag_for_faction(entry, buffer)?;
            } else if does_text_refer_to_a_space(entry) {
                write_output_tag_for_space(entry, buffer)?;
            } else if does_text_refer_to_a_faction_stat(entry) {
                write_output_tag_for_faction_stat(entry, buffer)?;
            } else if does_text_refer_to_forces(entry) {
                write_output_tag_for_forces(entry, buffer)?;
            } else if does_text_refer_to_control_types(entry) {
                write_output_tag_for_control_types(entry, buffer)?;
            } else if does_text_refer_to_support_levels(entry) {
                write_output_tag_for_support_levels(entry, buffer)?;
            } else if replace_extraneous_characters_from_text(entry)
                .parse::<u8>()
                .is_ok()
            {
                write_output_tag_for_numbers(entry, buffer)?;
            } else {
                write_regular_text(entry, buffer)?;
            }
        }

        reset_console_output_to_normal(buffer)?;

        Ok(buffer)
    }
}
