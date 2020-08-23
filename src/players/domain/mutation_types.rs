#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationTypes {
    Increase,
    Reduce,
    Flip,
    Move,
    Set,
    Unset,
    ShiftUp,
    ShiftDown,
}
