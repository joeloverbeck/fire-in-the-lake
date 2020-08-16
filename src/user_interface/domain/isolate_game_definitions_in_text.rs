use user_interface::domain::checking_game_definitions::does_text_unit_have_special_meaning::does_text_unit_have_special_meaning;

pub fn isolate_game_definitions_in_text(text: &str) -> Result<Vec<String>, String> {
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

    Ok(separated_text)
}
