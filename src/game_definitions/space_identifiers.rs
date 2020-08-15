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
    QuangTinQuangNgai,
    QuangDucLongKhanh,
    BinhTuyBinhThuan,
    Hue,
    PleikuDarlac,
    CentralLaos,
    SouthernLaos,
}

impl Eq for SpaceIdentifiers {}
