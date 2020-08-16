use text_manipulation::replace_extraneous_characters_from_text::replace_extraneous_characters_from_text;

pub fn does_text_refer_to_forces(text: &str) -> bool {
    // Strip text of possible extraneous characters.
    let mut filtered_text = replace_extraneous_characters_from_text(text);

    filtered_text = filtered_text.to_lowercase();

    if filtered_text == "usbase"
        || filtered_text == "vcbase"
        || filtered_text == "arvnbase"
        || filtered_text == "nvabase"
        || filtered_text == "usbase"
        || filtered_text == "tunneledvcbase"
        || filtered_text == "tunnelednvabase"
        || filtered_text == "ustroop"
        || filtered_text == "undergroundusirregular"
        || filtered_text == "activeusirregular"
        || filtered_text == "undergroundarvnranger"
        || filtered_text == "activearvnranger"
        || filtered_text == "arvntroop"
        || filtered_text == "arvnpolice"
        || filtered_text == "undergroundvcguerrilla"
        || filtered_text == "activevcguerrilla"
        || filtered_text == "activenvaguerrilla"
        || filtered_text == "undergroundnvaguerrilla"
    {
        return true;
    }

    false
}
