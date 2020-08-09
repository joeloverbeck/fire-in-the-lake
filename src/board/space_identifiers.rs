#[derive(Hash, Debug, Clone, Copy, PartialEq)]
pub enum SpaceIdentifiers {
    Saigon,
    AnLoc,
    KienGiangAnXuyen,
    BaXuyen,
    QuangNam,
    Route4,
    CanTho,
    Mekong,
    KienPhong,
}

impl Eq for SpaceIdentifiers {}

pub fn translate_space_name_to_identifier(space_name: &str) -> SpaceIdentifiers {
    if space_name == "saigon" {
        SpaceIdentifiers::Saigon
    } else if space_name == "an loc" {
        SpaceIdentifiers::AnLoc
    } else if space_name == "can tho" {
        SpaceIdentifiers::CanTho
    } else {
        todo!()
    }
}
