use board::space_identifiers::SpaceIdentifiers;

pub fn transform_typed_space_into_space_identifier(typed_space: &str) -> SpaceIdentifiers {
    if typed_space == "saigon" {
        SpaceIdentifiers::Saigon
    } else if typed_space == "an loc" {
        SpaceIdentifiers::AnLoc
    } else if typed_space == "can tho" {
        SpaceIdentifiers::CanTho
    } else if typed_space == "north vietnam" {
        SpaceIdentifiers::NorthVietnam
    } else if typed_space == "the parrot's beak" {
        SpaceIdentifiers::TheParrotsBeak
    } else if typed_space == "kien phong" {
        SpaceIdentifiers::KienPhong
    } else if typed_space == "kien giang" {
        SpaceIdentifiers::KienGiangAnXuyen
    } else if typed_space == "quang tri" {
        SpaceIdentifiers::QuangTriThuaThien
    } else if typed_space == "binh dinh" {
        SpaceIdentifiers::BinhDinh
    } else if typed_space == "pleiku" {
        SpaceIdentifiers::PleikuDarlac
    } else if typed_space == "hue" {
        SpaceIdentifiers::Hue
    } else if typed_space == "quang tin" {
        SpaceIdentifiers::QuangTinQuangNgai
    } else if typed_space == "quang duc" {
        SpaceIdentifiers::QuangDucLongKhanh
    } else if typed_space == "binh tuy" {
        SpaceIdentifiers::BinhTuyBinhThuan
    } else if typed_space == "central laos" {
        SpaceIdentifiers::CentralLaos
    } else if typed_space == "southern laos" {
        SpaceIdentifiers::SouthernLaos
    } else {
        panic!("Not implemented for {:?}", typed_space);
    }
}
