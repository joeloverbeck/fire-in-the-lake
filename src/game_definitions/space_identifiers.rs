use std::fmt;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
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
    QuiNhon,
    KhanhHoa,
    KienHoaVinhBinh,
    PhuBonPhuYen,
    TayNinh,
    Kontum,
    DaNang,
    CamRanh,
    OutOfPlay,
    Available,
    MekongNorth,
}

impl fmt::Display for SpaceIdentifiers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpaceIdentifiers::Saigon => write!(f, "[Saigon]"),
            SpaceIdentifiers::AnLoc => write!(f, "[AnLoc]"),
            SpaceIdentifiers::KienGiangAnXuyen => write!(f, "[KienGiangAnXuyen]"),
            SpaceIdentifiers::BaXuyen => write!(f, "[BaXuyen]"),
            SpaceIdentifiers::QuangNam => write!(f, "[QuangNam]"),
            SpaceIdentifiers::BinhDinh => write!(f, "[BinhDinh]"),
            SpaceIdentifiers::Route4 => write!(f, "[Route4]"),
            SpaceIdentifiers::CanTho => write!(f, "[CanTho]"),
            SpaceIdentifiers::Mekong => write!(f, "[Mekong]"),
            SpaceIdentifiers::KienPhong => write!(f, "[KienPhong]"),
            SpaceIdentifiers::QuangTriThuaThien => write!(f, "[QuangTriThuaThien]"),
            SpaceIdentifiers::NorthVietnam => write!(f, "[NorthVietnam]"),
            SpaceIdentifiers::TheParrotsBeak => write!(f, "[TheParrotsBeak]"),
            SpaceIdentifiers::QuangTinQuangNgai => write!(f, "[QuangTinQuangNgai]"),
            SpaceIdentifiers::QuangDucLongKhanh => write!(f, "[QuangDucLongKhanh]"),
            SpaceIdentifiers::BinhTuyBinhThuan => write!(f, "[BinhTuyBinhThuan]"),
            SpaceIdentifiers::Hue => write!(f, "[Hue]"),
            SpaceIdentifiers::PleikuDarlac => write!(f, "[PleikuDarlac]"),
            SpaceIdentifiers::CentralLaos => write!(f, "[CentralLaos]"),
            SpaceIdentifiers::SouthernLaos => write!(f, "[SouthernLaos]"),
            SpaceIdentifiers::QuiNhon => write!(f, "[QuiNhon]"),
            SpaceIdentifiers::KhanhHoa => write!(f, "[KhanhHoa]"),
            SpaceIdentifiers::KienHoaVinhBinh => write!(f, "[KienHoaVinhBinh]"),
            SpaceIdentifiers::PhuBonPhuYen => write!(f, "[PhuBonPhuYen]"),
            SpaceIdentifiers::TayNinh => write!(f, "[TayNinh]"),
            SpaceIdentifiers::Kontum => write!(f, "[Kontum]"),
            SpaceIdentifiers::DaNang => write!(f, "[DaNang]"),
            SpaceIdentifiers::CamRanh => write!(f, "[CamRanh]"),
            SpaceIdentifiers::MekongNorth => write!(f, "[MekongNorth]"),
            SpaceIdentifiers::OutOfPlay => write!(f, "Out of Play"),
            SpaceIdentifiers::Available => write!(f, "Available"),
        }
    }
}
