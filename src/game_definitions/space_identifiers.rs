use std::fmt;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpaceIdentifiers {
    Saigon,
    AnLoc,
    PhuocLong,
    KienGiangAnXuyen,
    BaXuyen,
    QuangNam,
    BinhDinh,
    CanTho,
    KienPhong,
    QuangTriThuaThien,
    NorthVietnam,
    TheParrotsBeak,
    TheFishhook,
    NortheastCambodia,
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
    MekongNorth,
    MekongSouth,
    Route1North,
    Route1NorthEast,
    Route1East,
    Route1South,
    Route1SouthEast,
    Route4West,
    Route4MekongEast,
    Route11,
    Route14North,
    Route14CentralNorth,
    Route14Central,
    Route14South,
    Route19,
    Route20,
    Route21,
    Sihanoukville,
    OutOfPlay,
    Available,
    Casualties,
}

impl fmt::Display for SpaceIdentifiers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpaceIdentifiers::Saigon => write!(f, "[Saigon]"),
            SpaceIdentifiers::AnLoc => write!(f, "[AnLoc]"),
            SpaceIdentifiers::PhuocLong => write!(f, "[PhuocLong]"),
            SpaceIdentifiers::KienGiangAnXuyen => write!(f, "[KienGiangAnXuyen]"),
            SpaceIdentifiers::BaXuyen => write!(f, "[BaXuyen]"),
            SpaceIdentifiers::QuangNam => write!(f, "[QuangNam]"),
            SpaceIdentifiers::BinhDinh => write!(f, "[BinhDinh]"),
            SpaceIdentifiers::CanTho => write!(f, "[CanTho]"),
            SpaceIdentifiers::MekongSouth => write!(f, "[MekongSouth]"),
            SpaceIdentifiers::KienPhong => write!(f, "[KienPhong]"),
            SpaceIdentifiers::QuangTriThuaThien => write!(f, "[QuangTriThuaThien]"),
            SpaceIdentifiers::NorthVietnam => write!(f, "[NorthVietnam]"),
            SpaceIdentifiers::TheParrotsBeak => write!(f, "[TheParrotsBeak]"),
            SpaceIdentifiers::TheFishhook => write!(f, "[TheFishhook]"),
            SpaceIdentifiers::NortheastCambodia => write!(f, "[NortheastCambodia]"),
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
            SpaceIdentifiers::Route1North => write!(f, "[Route1North]"),
            SpaceIdentifiers::Route1NorthEast => write!(f, "[Route1NorthEast]"),
            SpaceIdentifiers::Route1East => write!(f, "[Route1East]"),
            SpaceIdentifiers::Route1South => write!(f, "[Route1South]"),
            SpaceIdentifiers::Route1SouthEast => write!(f, "[Route1SouthEast]"),
            SpaceIdentifiers::Route4West => write!(f, "[Route4West]"),
            SpaceIdentifiers::Route4MekongEast => write!(f, "[Route4MekongEast]"),
            SpaceIdentifiers::Route11 => write!(f, "[Route11]"),
            SpaceIdentifiers::Route14North => write!(f, "[Route14North]"),
            SpaceIdentifiers::Route14CentralNorth => write!(f, "[Route14CentralNorth]"),
            SpaceIdentifiers::Route14Central => write!(f, "[Route14Central]"),
            SpaceIdentifiers::Route14South => write!(f, "[Route14South]"),
            SpaceIdentifiers::Route19 => write!(f, "[Route19]"),
            SpaceIdentifiers::Route20 => write!(f, "[Route20]"),
            SpaceIdentifiers::Route21 => write!(f, "[Route21]"),
            SpaceIdentifiers::Sihanoukville => write!(f, "[Sihanoukville]"),
            SpaceIdentifiers::OutOfPlay => write!(f, "Out of Play"),
            SpaceIdentifiers::Available => write!(f, "Available"),
            SpaceIdentifiers::Casualties => write!(f, "Casualties"),
        }
    }
}
