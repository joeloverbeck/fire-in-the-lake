use text_manipulation::replace_extraneous_characters_from_text::replace_extraneous_characters_from_text;

pub fn does_text_refer_to_a_space(text: &str) -> bool {
    // Strip text of possible extraneous characters.
    let mut filtered_text = replace_extraneous_characters_from_text(text);

    filtered_text = filtered_text.to_lowercase();

    if filtered_text == "saigon"
        || filtered_text == "hue"
        || filtered_text == "anloc"
        || filtered_text == "quinhon"
        || filtered_text == "camranh"
        || filtered_text == "cantho"
        || filtered_text == "danang"
        || filtered_text == "kontum"
        || filtered_text == "quangtrithuathien"
        || filtered_text == "binhdinh"
        || filtered_text == "quangnam"
        || filtered_text == "pleikudarlac"
        || filtered_text == "quangtinquangngai"
        || filtered_text == "quangduclongkhanh"
        || filtered_text == "binhtuybinhthuan"
        || filtered_text == "tayninh"
        || filtered_text == "phubonphuyen"
        || filtered_text == "khanhhoa"
        || filtered_text == "kienhoavinhbinh"
        || filtered_text == "baxuyen"
        || filtered_text == "kienphong"
        || filtered_text == "kiengianganxuyen"
        || filtered_text == "northvietnam"
        || filtered_text == "centrallaos"
        || filtered_text == "southernlaos"
        || filtered_text == "theparrotsbeak"
    {
        return true;
    }

    false
}
