#[cfg(feature = "mock")]
pub mod mock;
pub trait IsVisualProtocol: 'static {
    type VisualComponent: std::fmt::Debug
        + Clone
        + PartialEq
        + Eq
        + Send
        + serde::Serialize
        + for<'a> serde::Deserialize<'a>
        + 'static;

    type Visual: IsVisual;
}

pub trait IsVisual {
    type Component;

    fn from_components(components: &[Self::Component]) -> Self;
}

pub type VisualComponent<VisualProtocol> = <VisualProtocol as IsVisualProtocol>::VisualComponent;
