use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum Factions {
    None,
    VC,
    NVA,
    ARVN,
    US,
}

impl fmt::Debug for Factions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Factions::None => write!(f, "None"),
            Factions::VC => write!(f, "VC"),
            Factions::NVA => write!(f, "NVA"),
            Factions::ARVN => write!(f, "ARVN"),
            Factions::US => write!(f, "US"),
        }
    }
}

impl fmt::Display for Factions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Factions::None => write!(f, "None"),
            Factions::VC => write!(f, "VC"),
            Factions::NVA => write!(f, "NVA"),
            Factions::ARVN => write!(f, "ARVN"),
            Factions::US => write!(f, "US"),
        }
    }
}
