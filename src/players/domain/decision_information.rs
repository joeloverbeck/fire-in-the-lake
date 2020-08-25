#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecisionInformation {
    Event,
    Pass,
    Attack,
    Rally,
    Ambush,
    Terror,
    Bombard,
}
