#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Choices {
    OperationOnly,
    ShadedEvent,
    SecondLimitedOperation,
    SecondOperationAndSpecialActivity,
    Pass,
}
