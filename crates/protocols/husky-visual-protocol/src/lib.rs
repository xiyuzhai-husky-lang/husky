use serde::{Deserialize, Serialize};

#[cfg(feature = "mock")]
pub mod mock;
#[cfg(feature = "trivial")]
pub mod trivial;

/// require `Serialize` and `Deserialize` for the convenience of deriving `Serialize` and `Deserialize` for generic types
///
/// for example TraceCache
pub trait IsVisualProtocol:
    std::fmt::Debug
    + Default
    + PartialEq
    + Eq
    + Clone
    + Serialize
    + for<'a> Deserialize<'a>
    + Send
    + 'static
{
    type VisualComponent: IsVisualComponent;

    type Visual: IsVisual<Component = Self::VisualComponent>;
}

pub trait IsVisualComponent:
    std::fmt::Debug
    + Clone
    + PartialEq
    + Eq
    + Send
    + serde::Serialize
    + for<'a> serde::Deserialize<'a>
    + 'static
{
}

impl IsVisualComponent for () {}

pub trait IsVisual {
    type Component;

    fn from_components(components: &[Self::Component]) -> Self;
}
