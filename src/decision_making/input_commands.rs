use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputCommands {
    Event,
    Pass,
    Operation,
    OperationOnly,
    Govern,
    Rally,
    Sweep,
    Train,
    Tax,
    Stop,
    Yes,
    No,
    Saigon,
    Hue,
    AnLoc,
    CanTho,
    NorthVietnam,
    TheParrotsBeak,
    KienPhong,
    KienGiang,
    QuangTri,
    BinhDinh,
    PleikuDarlac,
    QuangTinQuangNgai,
    QuangDucLongKhanh,
    BinhTuyBinhThuan,
    Six,
}

impl fmt::Display for InputCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
