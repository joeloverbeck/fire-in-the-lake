#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecisionInformation {
    Event,
    Pass,
    Attack,
    Ambush,
    Terror,
    Bombard,
}
