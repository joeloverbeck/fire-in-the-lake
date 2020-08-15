use text_manipulation::replace_extraneous_characters_from_text::replace_extraneous_characters_from_text;
use user_interface::domain::does_text_refer_to_a_faction_stat::does_text_refer_to_a_faction_stat;
use user_interface::domain::does_text_refer_to_forces::does_text_refer_to_forces;
use user_interface::domain::does_text_refer_to_space::does_text_refer_to_a_space;

pub fn does_text_unit_have_special_meaning(text: &str) -> bool {
    text == "[VC]"
        || text == "[ARVN]"
        || text == "[US]"
        || text == "[NVA]"
        || does_text_refer_to_a_space(text)
        || does_text_refer_to_a_faction_stat(text)
        || does_text_refer_to_forces(text)
        || replace_extraneous_characters_from_text(text)
            .parse::<u8>()
            .is_ok()
}
