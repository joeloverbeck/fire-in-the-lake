#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationTypes {
    Increase,
    Reduce,
    Move,
}
