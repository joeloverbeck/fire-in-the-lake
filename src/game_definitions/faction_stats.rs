use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FactionStats {
    Aid,
    TotalEcon,
    Patronage,
    ArvnResources,
    NvaResources,
    VcResources,
    SupportPlusAvailable,
    CoinPlusPatronage,
    OppositionPlusBases,
    NvaPlusBases,
    TheTrail,
}

impl fmt::Display for FactionStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FactionStats::Aid => write!(f, "[Aid]"),
            FactionStats::TotalEcon => write!(f, "[TotalEcon]"),
            FactionStats::Patronage => write!(f, "[Patronage]"),
            FactionStats::ArvnResources => write!(f, "[ArvnResources]"),
            FactionStats::NvaResources => write!(f, "[NvaResources]"),
            FactionStats::VcResources => write!(f, "[VcResources]"),
            FactionStats::SupportPlusAvailable => write!(f, "[SupportPlusAvailable]"),
            FactionStats::CoinPlusPatronage => write!(f, "[CoinPlusPatronage]"),
            FactionStats::OppositionPlusBases => write!(f, "[OppositionPlusBases]"),
            FactionStats::NvaPlusBases => write!(f, "[NvaPlusBases]"),
            FactionStats::TheTrail => write!(f, "[TheTrail]"),
        }
    }
}
