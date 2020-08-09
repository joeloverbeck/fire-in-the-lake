#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Choices {
    OperationOnly,
    UnshadedEvent,
    ShadedEvent,
    SecondLimitedOperation,
    SecondOperationAndSpecialActivity,
    Pass,
}
