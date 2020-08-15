use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SupportLevels {
    ActiveSupport,
    PassiveSupport,
    Neutral,
    PassiveOpposition,
    ActiveOpposition,
}

impl fmt::Display for SupportLevels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SupportLevels::ActiveSupport => write!(f, "[ActiveSupport]"),
            SupportLevels::PassiveSupport => write!(f, "[PassiveSupport]"),
            SupportLevels::Neutral => write!(f, "[Neutral]"),
            SupportLevels::PassiveOpposition => write!(f, "[PassiveOpposition]"),
            SupportLevels::ActiveOpposition => write!(f, "[ActiveOpposition]"),
        }
    }
}
