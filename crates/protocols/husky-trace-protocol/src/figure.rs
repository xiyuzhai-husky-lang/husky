use crate::TraceId;
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};
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
    fn new_specific(
        followed_visual: Option<(TraceId, Visual)>,
        accompanying_visuals: impl Iterator<Item = (TraceId, Visual)>,
    ) -> Self;

    type ViewCache;

    type View<'a>;

    fn view<'a>(
        &'a self,
        sct: &'a VisualSynchrotron,
        view_cache: &mut Self::ViewCache,
    ) -> Self::View<'a>;
}

impl IsFigure for () {
    fn new_specific(
        followed_visual: Option<(TraceId, Visual)>,
        accompanying_visuals: impl Iterator<Item = (TraceId, Visual)>,
    ) -> Self {
        ()
    }

    type View<'a> = ();

    fn view<'a>(&'a self, sct: &'a VisualSynchrotron) -> Self::View<'a> {
        ()
    }
}
