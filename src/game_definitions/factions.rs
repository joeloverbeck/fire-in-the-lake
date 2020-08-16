use std::fmt;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Factions {
    VC,
    ARVN,
    NVA,
    US,
}

impl fmt::Display for Factions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Factions::US => write!(f, "[US]"),
            Factions::ARVN => write!(f, "[ARVN]"),
            Factions::VC => write!(f, "[VC]"),
            Factions::NVA => write!(f, "[NVA]"),
        }
    }
}
