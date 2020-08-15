use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Forces {
    UsBase,
    NvaBase,
    TunneledNvaBase,
    VcBase,
    TunneledVcBase,
    ArvnBase,
    UsTroop,
    UndergroundUsIrregular,
    ActiveUsIrregular,
    ArvnTroop,
    UndergroundArvnRanger,
    ActiveArvnRanger,
    ArvnPolice,
    UndergroundVcGuerrilla,
    ActiveVcGuerrilla,
    UndergroundNvaGuerrilla,
    ActiveNvaGuerrilla,
}

impl fmt::Display for Forces {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Forces::UsBase => write!(f, "[UsBase]"),
            Forces::NvaBase => write!(f, "[NvaBase]"),
            Forces::TunneledNvaBase => write!(f, "[TunneledNvaBase]"),
            Forces::TunneledVcBase => write!(f, "[TunneledVcBase]"),
            Forces::VcBase => write!(f, "[VcBase]"),
            Forces::ArvnBase => write!(f, "[ArvnBase]"),
            Forces::UsTroop => write!(f, "[UsTroop]"),
            Forces::UndergroundUsIrregular => write!(f, "[UndergroundUsIrregular]"),
            Forces::ActiveUsIrregular => write!(f, "[ActiveUsIrregular]"),
            Forces::ArvnTroop => write!(f, "[ArvnTroop]"),
            Forces::UndergroundArvnRanger => write!(f, "[UndergroundArvnRanger]"),
            Forces::ActiveArvnRanger => write!(f, "[ActiveArvnRanger]"),
            Forces::ArvnPolice => write!(f, "[ArvnPolice]"),
            Forces::UndergroundVcGuerrilla => write!(f, "[UndergroundVcGuerrilla]"),
            Forces::ActiveVcGuerrilla => write!(f, "[ActiveVcGuerrilla]"),
            Forces::UndergroundNvaGuerrilla => write!(f, "[UndergroundNvaGuerrilla]"),
            Forces::ActiveNvaGuerrilla => write!(f, "[ActiveNvaGuerrilla]"),
        }
    }
}
