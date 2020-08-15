use text_manipulation::replace_extraneous_characters_from_text::replace_extraneous_characters_from_text;

pub fn does_text_refer_to_a_faction_stat(text: &str) -> bool {
    // Strip text of possible extraneous characters.
    let mut filtered_text = replace_extraneous_characters_from_text(text);

    filtered_text = filtered_text.to_lowercase();

    if filtered_text == "aid"
        || filtered_text == "totalecon"
        || filtered_text == "patronage"
        || filtered_text == "arvnresources"
        || filtered_text == "vcresources"
        || filtered_text == "nvaresources"
        || filtered_text == "supportplusavailable"
        || filtered_text == "coinpluspatronage"
        || filtered_text == "oppositionplusbases"
        || filtered_text == "nvaplusbases"
        || filtered_text == "thetrail"
    {
        return true;
    }

    false
}
