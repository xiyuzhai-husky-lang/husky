//! Daniel Jackson is a character in Stargate SG1.
//!
//! He is a linguist.
//!
//! So this visualization serves mainly for linguistics.
pub mod action;

use husky_trace_protocol::{figure::IsFigure, id::TraceId};
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DanielJacksonFigure;

impl IsFigure for DanielJacksonFigure {
    fn new_specific(
        followed_visual: Option<(TraceId, Visual)>,
        accompanying_visuals: impl IntoIterator<Item = (TraceId, Visual)>,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self {
        todo!()
    }
}
