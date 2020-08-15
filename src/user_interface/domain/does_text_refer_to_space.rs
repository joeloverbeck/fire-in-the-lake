use text_manipulation::replace_extraneous_characters_from_text::replace_extraneous_characters_from_text;

pub fn does_text_refer_to_a_space(text: &str) -> bool {
    // Strip text of possible extraneous characters.
    let mut filtered_text = replace_extraneous_characters_from_text(text);

    filtered_text = filtered_text.to_lowercase();

    if filtered_text == "saigon" {
        return true;
    }

    false
}
