use decision_making::input_commands::InputCommands;

#[derive(Hash, Debug, Clone, Copy, PartialEq)]
pub enum SpaceIdentifiers {
    Saigon,
    AnLoc,
    KienGiangAnXuyen,
    BaXuyen,
    QuangNam,
    BinhDinh,
    Route4,
    CanTho,
    Mekong,
    KienPhong,
    QuangTriThuaThien,
    NorthVietnam,
    TheParrotsBeak,
}

impl Eq for SpaceIdentifiers {}

pub fn translate_space_name_to_identifier(space_name: InputCommands) -> SpaceIdentifiers {
    if space_name == InputCommands::Saigon {
        SpaceIdentifiers::Saigon
    } else if space_name == InputCommands::AnLoc {
        SpaceIdentifiers::AnLoc
    } else if space_name == InputCommands::CanTho {
        SpaceIdentifiers::CanTho
    } else if space_name == InputCommands::NorthVietnam {
        SpaceIdentifiers::NorthVietnam
    } else if space_name == InputCommands::TheParrotsBeak {
        SpaceIdentifiers::TheParrotsBeak
    } else if space_name == InputCommands::KienPhong {
        SpaceIdentifiers::KienPhong
    } else if space_name == InputCommands::KienGiang {
        SpaceIdentifiers::KienGiangAnXuyen
    } else if space_name == InputCommands::QuangTri {
        SpaceIdentifiers::QuangTriThuaThien
    } else if space_name == InputCommands::BinhDinh {
        SpaceIdentifiers::BinhDinh
    } else {
        todo!()
    }
}
