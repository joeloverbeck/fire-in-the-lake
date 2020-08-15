use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlTypes {
    Coin,
    Uncontrolled,
    Nva,
}

impl fmt::Display for ControlTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ControlTypes::Coin => write!(f, "[Coin]"),
            ControlTypes::Uncontrolled => write!(f, "[Uncontrolled]"),
            ControlTypes::Nva => write!(f, "[Nva]"),
        }
    }
}
