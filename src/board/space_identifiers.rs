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
    QuangTriThuaThien,
    NorthVietnam,
    TheParrotsBeak,
}

impl Eq for SpaceIdentifiers {}

pub fn translate_space_name_to_identifier(space_name: &str) -> SpaceIdentifiers {
    if space_name == "saigon" {
        SpaceIdentifiers::Saigon
    } else if space_name == "an loc" {
        SpaceIdentifiers::AnLoc
    } else if space_name == "can tho" {
        SpaceIdentifiers::CanTho
    } else if space_name == "north vietnam" {
        SpaceIdentifiers::NorthVietnam
    } else if space_name == "the parrot's beak" {
        SpaceIdentifiers::TheParrotsBeak
    } else if space_name == "kien phong" {
        SpaceIdentifiers::KienPhong
    } else if space_name == "kien giang" {
        SpaceIdentifiers::KienGiangAnXuyen
    } else if space_name == "quang tri" {
        SpaceIdentifiers::QuangTriThuaThien
    } else {
        todo!()
    }
}
