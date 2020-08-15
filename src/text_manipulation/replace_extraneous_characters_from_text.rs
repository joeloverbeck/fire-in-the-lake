pub fn replace_extraneous_characters_from_text(text: &str) -> String {
    text.replace(
        &['(', ')', ',', '\"', '.', ';', ':', '\'', '{', '}'][..],
        "",
    )
}
