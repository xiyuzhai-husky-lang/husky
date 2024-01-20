use serde::{Deserialize, Serialize};

/// `IsFigure` extends `Serialize` and `Deserialize` for the convenience of deriving `Serialize` and `Deserialize` for generic types
///
/// for example TraceSynchrotron
pub trait IsFigure:
    Default
    + std::fmt::Debug
    + Default
    + PartialEq
    + Eq
    + Clone
    + Serialize
    + for<'a> Deserialize<'a>
    + Send
    + 'static
{
    /// construct a figure for a specific datapoint
    fn new_specific() -> Self;
}

impl IsFigure for () {
    fn new_specific() -> Self {
        ()
    }
}
