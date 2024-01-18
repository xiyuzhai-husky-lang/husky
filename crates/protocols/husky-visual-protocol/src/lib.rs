pub mod synchrotron;

use serde::{Deserialize, Serialize};

#[cfg(feature = "mock")]
pub mod mock;

/// require `Serialize` and `Deserialize` for the convenience of deriving `Serialize` and `Deserialize` for generic types
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
}

impl IsFigure for () {}
