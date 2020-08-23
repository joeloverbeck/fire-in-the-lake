#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Movements {
    FirstEligible,
    SecondEligible,
    FirstFactionEvent,
    FirstFactionOperationOnly,
    Passed,
}
