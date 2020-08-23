#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpaceMutationValues {
    ActiveSupport,
    PassiveSupport,
    Neutral,
    PassiveOpposition,
    ActiveOpposition,
    CoinControl,
    Uncontrolled,
    NvaControl,
}
