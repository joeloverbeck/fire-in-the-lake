use game_definitions::space_identifiers::SpaceIdentifiers;

pub fn translate_typed_input_to_space_identifier(
    typed_input: &str,
) -> Result<SpaceIdentifiers, String> {
    let typed_input_to_lowercase = typed_input.to_lowercase();

    if typed_input_to_lowercase == "saigon" || typed_input_to_lowercase == "sai" {
        Ok(SpaceIdentifiers::Saigon)
    } else if typed_input_to_lowercase == "an loc" {
        Ok(SpaceIdentifiers::AnLoc)
    } else if typed_input_to_lowercase == "out of play" || typed_input_to_lowercase == "out" {
        Ok(SpaceIdentifiers::OutOfPlay)
    } else if typed_input_to_lowercase == "available" {
        Ok(SpaceIdentifiers::Available)
    } else if typed_input_to_lowercase == "kien giang" {
        Ok(SpaceIdentifiers::KienGiangAnXuyen)
    } else if typed_input_to_lowercase == "ba xuyen" || typed_input_to_lowercase == "ba" {
        Ok(SpaceIdentifiers::BaXuyen)
    } else if typed_input_to_lowercase == "quang nam" {
        Ok(SpaceIdentifiers::QuangNam)
    } else {
        panic!("Not implemented for typed_input: {:?}", typed_input);
    }
}
