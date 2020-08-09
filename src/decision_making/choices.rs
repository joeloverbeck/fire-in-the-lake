#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Choices {
    OperationOnly,
    ShadedEvent,
    SecondOperationAndSpecialActivity,
    Pass,
}
