pub fn form_typed_space(typed_space_with_underscores: &str) -> String {
    // It would receive something like this: "hue_city"
    // The goal is to split that into "hue", "city"
    // Then merge both into a string without adding a space at the end.
    let split_typed_space_with_underscores: Vec<&str> =
        typed_space_with_underscores.split('_').collect();

    let mut composed_typed_space: String = "".to_string();

    for (index, word) in split_typed_space_with_underscores.iter().enumerate() {
        composed_typed_space = composed_typed_space + word;

        if index < split_typed_space_with_underscores.len() - 1 {
            composed_typed_space += " ";
        }
    }

    composed_typed_space
}
