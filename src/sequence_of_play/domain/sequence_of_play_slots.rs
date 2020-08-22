use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SequenceOfPlaySlots {
    Pass,
    FirstFactionOperationOnly,
    FirstFactionOperationPlusSpecialActivity,
    FirstFactionEvent,
    SecondFactionLimitedOperation,
    SecondFactionLimitedOperationOrEvent,
    SecondFactionOperationPlusSpecialActivity,
}

impl fmt::Display for SequenceOfPlaySlots {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SequenceOfPlaySlots::Pass => write!(f, "[pass]"),
            SequenceOfPlaySlots::FirstFactionOperationOnly => write!(f, "[operation only]"),
            SequenceOfPlaySlots::FirstFactionOperationPlusSpecialActivity => {
                write!(f, "[operation]")
            }
            SequenceOfPlaySlots::FirstFactionEvent => write!(f, "[event]"),
            SequenceOfPlaySlots::SecondFactionLimitedOperation => write!(f, "[limited operation]"),
            SequenceOfPlaySlots::SecondFactionLimitedOperationOrEvent => {
                write!(f, "[limited operation] [event]")
            }
            SequenceOfPlaySlots::SecondFactionOperationPlusSpecialActivity => {
                write!(f, "[operation]")
            }
        }
    }
}
